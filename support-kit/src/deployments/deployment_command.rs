use clap::Parser;

use crate::SupportControl;

use super::DeploymentContext;

#[derive(Clone, Copy, Parser, Debug, PartialEq)]
#[clap(rename_all = "kebab-case")]
pub enum DeploymentCommand {
    Install,
    Setup,
    Build,
    Start,
    List,
    Login,
    Pull,
    Push,
    Restart,
}

impl DeploymentCommand {
    #[tracing::instrument(skip(self, controller), level = "trace")]
    pub async fn exec_remote(&self, controller: &SupportControl) -> crate::Result<()> {
        Ok(exec_remote_container_op(&controller, *self).await?)
    }

    #[tracing::instrument(skip(self, controller), level = "trace")]
    pub async fn exec_local(&self, controller: &SupportControl) -> crate::Result<()> {
        Ok(exec_local_container_op(&controller, self).await?)
    }
}

#[tracing::instrument(skip(controller, command), level = "trace")]
pub async fn exec_local_container_op(
    controller: &SupportControl,
    command: &DeploymentCommand,
) -> crate::Result<()> {
    let deployment_context = DeploymentContext::from(controller);
    tracing::trace!(command = ?command, "executing local container operation");
    match command {
        DeploymentCommand::Build => {
            for image in deployment_context.images {
                image.build()?.run()?;
            }
        }
        DeploymentCommand::Install => {
            deployment_context.get_install_script()?.run()?;
            deployment_context.install_docker()?.run()?;
        }
        DeploymentCommand::List => deployment_context.list_containers()?.run()?,
        DeploymentCommand::Login => deployment_context.login()?.run()?,
        DeploymentCommand::Pull => {
            for image in deployment_context.images {
                image.pull()?.run()?;
            }
        }
        DeploymentCommand::Push => {
            for image in deployment_context.images {
                image.push()?.run()?;
            }
        }
        DeploymentCommand::Restart => {
            let path = deployment_context.emit_config()?;
            let path = if path.is_absolute() {
                path
            } else {
                std::env::current_dir()
                    .expect("unable to get current directory")
                    .join(path)
            };

            for image in deployment_context.images {
                image.pull()?.run()?;
                image.kill_all()?.run()?;
                image.start(path.clone())?.run()?;
            }
        }
        DeploymentCommand::Setup => {
            deployment_context.setup_cert_volume()?.run()?;

            for image in deployment_context.images {
                image.setup_log_volume()?.run()?;
                image.setup_data_volume()?.run()?;
                image.setup_config_volume()?.run()?;
            }
        }
        DeploymentCommand::Start => {
            let path = deployment_context.emit_config()?;
            let path = if path.is_absolute() {
                path
            } else {
                std::env::current_dir()
                    .expect("unable to get current directory")
                    .join(path)
            };

            for image in deployment_context.images {
                image.start(path.clone())?.run()?;
            }
        }
    }

    Ok(())
}

#[tracing::instrument(skip(controller, command), level = "trace")]
pub async fn exec_remote_container_op(
    controller: &SupportControl,
    command: DeploymentCommand,
) -> crate::Result<()> {
    let deployment_context = DeploymentContext::from(controller);
    tracing::trace!(command = ?command, "executing remote container operation");
    match command {
        DeploymentCommand::Build => {
            for image in deployment_context.images {
                controller
                    .on_remotes()
                    .commands(image.build()?)
                    .call()
                    .await?;
            }
        }
        DeploymentCommand::Install => {
            controller
                .on_remotes()
                .commands(bon::vec![
                    deployment_context.get_install_script()?,
                    deployment_context.install_docker()?
                ])
                .call()
                .await?;
        }
        DeploymentCommand::List => {
            controller
                .on_remotes()
                .commands(deployment_context.list_containers()?)
                .call()
                .await?;
        }
        DeploymentCommand::Login => {
            controller
                .on_remotes()
                .commands(deployment_context.login()?)
                .call()
                .await?;
        }
        DeploymentCommand::Pull => {
            for image in deployment_context.images {
                controller
                    .on_remotes()
                    .commands(image.pull()?)
                    .call()
                    .await?;
            }
        }
        DeploymentCommand::Push => {}
        DeploymentCommand::Restart => {
            let path = deployment_context.emit_config()?;
            let from_path = path.to_string_lossy();
            let to_path = format!("./{name}.json", name = controller.config.name());

            for host in deployment_context.hosts {
                host.send_file(&from_path, &to_path)?.run()?;
            }

            for image in deployment_context.images {
                controller
                    .on_remotes()
                    .commands(bon::vec![
                        image.pull()?,
                        image.kill_all()?,
                        image.start(to_path.clone())?
                    ])
                    .call()
                    .await?;
            }
        }
        DeploymentCommand::Setup => {
            let certs_volume = deployment_context.setup_cert_volume()?;

            for image in deployment_context.images {
                controller
                    .on_remotes()
                    .commands(bon::vec![
                        certs_volume.clone(),
                        image.setup_log_volume()?,
                        image.setup_data_volume()?,
                        image.setup_config_volume()?
                    ])
                    .call()
                    .await?;
            }
        }
        DeploymentCommand::Start => {
            let path = deployment_context.emit_config()?;

            let from_path = path.to_string_lossy();
            let to_path = format!("{name}.container.json", name = controller.config.name());

            for host in deployment_context.hosts {
                host.send_file(&from_path, &to_path)?.run()?;
            }

            for image_controller in deployment_context.images {
                controller
                    .on_remotes()
                    .commands(image_controller.start(to_path.clone())?)
                    .call()
                    .await?;
            }
        }
    }

    Ok(())
}
