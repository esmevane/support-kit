use clap::Parser;

use crate::{shell, SupportControl};

use super::ContainerCommands;

#[derive(Clone, Copy, Parser, Debug)]
#[clap(rename_all = "kebab-case")]
pub enum ContainerOps {
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

impl ContainerOps {
    pub async fn exec_remote(&self, controller: &SupportControl) -> crate::Result<()> {
        Ok(exec_remote_container_op(&controller, *self).await?)
    }

    pub async fn exec_local(&self, controller: &SupportControl) -> crate::Result<()> {
        Ok(exec_local_container_op(&controller, self).await?)
    }
}

pub async fn exec_local_container_op(
    controller: &SupportControl,
    command: &ContainerOps,
) -> crate::Result<()> {
    let container_commands = ContainerCommands::from(controller);
    match command {
        ContainerOps::Build => {
            for image_commands in container_commands.images {
                image_commands.build()?.run()?;
            }
        }
        ContainerOps::Install => {
            container_commands.get_install_script()?.run()?;
            container_commands.install_docker()?.run()?;
        }
        ContainerOps::List => container_commands.list_containers()?.run()?,
        ContainerOps::Login => container_commands.login()?.run()?,
        ContainerOps::Pull => {
            for image_commands in container_commands.images {
                image_commands.pull()?.run()?;
            }
        }
        ContainerOps::Push => {
            for image_commands in container_commands.images {
                image_commands.push()?.run()?;
            }
        }
        ContainerOps::Restart => {
            let path = container_commands.emit_config()?;
            for image_commands in container_commands.images {
                image_commands.pull()?.run()?;
                image_commands.kill_all()?.run()?;
                image_commands.start(path.clone())?.run()?;
            }
        }
        ContainerOps::Setup => {
            container_commands.setup_cert_volume()?.run()?;

            for image_commands in container_commands.images {
                image_commands.setup_config_volume()?.run()?;
            }
        }
        ContainerOps::Start => {
            let path = container_commands.emit_config()?;
            for image_commands in container_commands.images {
                image_commands.start(path.clone())?.run()?;
            }
        }
    }

    Ok(())
}

pub async fn exec_remote_container_op(
    controller: &SupportControl,
    command: ContainerOps,
) -> crate::Result<()> {
    let container_commands = ContainerCommands::from(controller);
    match command {
        ContainerOps::Build => {
            for image_controller in container_commands.images {
                controller
                    .on_remotes()
                    .commands(image_controller.build()?)
                    .call()
                    .await?;
            }
        }
        ContainerOps::Install => {
            controller
                .on_remotes()
                .commands(bon::vec![
                    container_commands.get_install_script()?,
                    container_commands.install_docker()?
                ])
                .call()
                .await?;
        }
        ContainerOps::List => {
            controller
                .on_remotes()
                .commands(container_commands.list_containers()?)
                .call()
                .await?;
        }
        ContainerOps::Login => {
            controller
                .on_remotes()
                .commands(container_commands.login()?)
                .call()
                .await?;
        }
        ContainerOps::Pull => {
            for image_controller in container_commands.images {
                controller
                    .on_remotes()
                    .commands(image_controller.pull()?)
                    .call()
                    .await?;
            }
        }
        ContainerOps::Push => {}
        ContainerOps::Restart => {
            let path = container_commands.emit_config()?;
            for image_controller in container_commands.images {
                controller
                    .on_remotes()
                    .commands(bon::vec![
                        image_controller.pull()?,
                        image_controller.kill_all()?,
                        image_controller.start(path.clone())?
                    ])
                    .call()
                    .await?;
            }
        }
        ContainerOps::Setup => {
            let certs_volume = container_commands.setup_cert_volume()?;

            for image_controller in container_commands.images {
                controller
                    .on_remotes()
                    .commands(bon::vec![
                        certs_volume.clone(),
                        image_controller.setup_config_volume()?
                    ])
                    .call()
                    .await?;
            }
        }
        ContainerOps::Start => {
            let path = container_commands.emit_config()?;

            let path = path.clone();
            let name = controller.config.name();

            controller
                .per_remote(|host_details| {
                    let name = name.clone();
                    shell(format!(
                        "scp -i {key} {local} {user}@{host}:{name}.json",
                        local = path.display(),
                        key = host_details.auth,
                        user = host_details.user,
                        host = host_details.address,
                    ))?
                    .run()?;

                    Ok(())
                })
                .await?;

            for image_controller in container_commands.images {
                controller
                    .on_remotes()
                    .commands(image_controller.start(path.clone())?)
                    .call()
                    .await?;
            }
        }
    }

    Ok(())
}

// pub async fn exec_per_host(
//     deployment: &Deployment,
//     local_path: impl AsRef<str>,
//     remote_path: impl AsRef<str>,
// ) -> Result<(), SshError> {
//     // scp -i ~/.ssh/private_key path/to/local_file remote_host:path/to/remote_file

//     for host in deployment.hosts.clone() {
//         let command = shell(format!(
//             "scp -i {key} {local} {user}@{host}:{remote}",
//             key = host.auth.unwrap_or("~/.ssh/id_rsa".into()),
//             local = local_path.as_ref(),
//             user = host.user.unwrap_or("root".into()),
//             host = host.address,
//             remote = remote_path.as_ref()
//         ))?;

//         command.run()?;
//     }

//     Ok(())
// }
