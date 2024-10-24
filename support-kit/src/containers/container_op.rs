use clap::Parser;

use crate::SupportControl;

use super::ContainerControl;

#[derive(Clone, Copy, Parser, Debug)]
#[clap(rename_all = "kebab-case")]
pub enum ContainerOp {
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

impl ContainerOp {
    pub async fn exec_remote(&self, controller: &SupportControl) -> crate::Result<()> {
        Ok(exec_remote_container_op(&controller, *self).await?)
    }

    pub async fn exec_local(&self, controller: &SupportControl) -> crate::Result<()> {
        Ok(exec_local_container_op(&controller, self).await?)
    }
}

pub async fn exec_local_container_op(
    controller: &SupportControl,
    command: &ContainerOp,
) -> crate::Result<()> {
    let container_ops = ContainerControl::from(controller);
    match command {
        ContainerOp::Setup => {
            container_ops.setup_cert_volume()?.run()?;

            for image in container_ops.images {
                image.setup_config_volume()?.run()?;
            }
        }
        ContainerOp::Install => {
            container_ops.get_install_script()?.run()?;
            container_ops.install_docker()?.run()?;
        }
        ContainerOp::Build => {
            for image in container_ops.images {
                image.build()?.run()?;
            }
        }
        ContainerOp::Start => {
            for image in container_ops.images {
                image.start()?.run()?;
            }
        }
        ContainerOp::List => {
            container_ops.list_containers()?.run()?;
        }
        ContainerOp::Login => {
            container_ops.login()?.run()?;
        }
        ContainerOp::Pull => {
            for image in container_ops.images {
                image.pull()?.run()?;
            }
        }
        ContainerOp::Push => {
            for image in container_ops.images {
                image.push()?.run()?;
            }
        }
        ContainerOp::Restart => {
            for image in container_ops.images {
                image.pull()?.run()?;
                image.kill_all()?.run()?;
                image.start()?.run()?;
            }
        }
    }

    Ok(())
}

pub async fn exec_remote_container_op(
    controller: &SupportControl,
    command: ContainerOp,
) -> crate::Result<()> {
    let container_ops = ContainerControl::from(controller);
    match command {
        ContainerOp::Setup => {
            let certs_volume = container_ops.setup_cert_volume()?;

            for image in container_ops.images {
                controller
                    .on_remotes()
                    .commands(bon::vec![
                        certs_volume.clone(),
                        image.setup_config_volume()?
                    ])
                    .call()
                    .await?;
            }
        }
        ContainerOp::Install => {
            controller
                .on_remotes()
                .commands(bon::vec![
                    container_ops.get_install_script()?,
                    container_ops.install_docker()?
                ])
                .call()
                .await?;
        }
        ContainerOp::Build => {
            for image in container_ops.images {
                controller
                    .on_remotes()
                    .commands(image.build()?)
                    .call()
                    .await?;
            }
        }
        ContainerOp::Start => {
            for image in container_ops.images {
                controller
                    .on_remotes()
                    .commands(image.start()?)
                    .call()
                    .await?;
            }
        }
        ContainerOp::List => {
            controller
                .on_remotes()
                .commands(container_ops.list_containers()?)
                .call()
                .await?;
        }
        ContainerOp::Login => {
            controller
                .on_remotes()
                .commands(container_ops.login()?)
                .call()
                .await?;
        }
        ContainerOp::Pull => {
            for image in container_ops.images {
                controller
                    .on_remotes()
                    .commands(image.pull()?)
                    .call()
                    .await?;
            }
        }
        ContainerOp::Restart => {
            for image in container_ops.images {
                controller
                    .on_remotes()
                    .commands(bon::vec![image.pull()?, image.kill_all()?, image.start()?])
                    .call()
                    .await?;
            }
        }
        ContainerOp::Push => {}
    }

    Ok(())
}
