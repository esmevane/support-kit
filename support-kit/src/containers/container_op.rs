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
                image.kill_all()?.run()?;
                image.pull()?.run()?;
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
                let config_volume = image.setup_config_volume()?;

                controller
                    .on_hosts(|connection| {
                        let certs_volume = certs_volume.clone();
                        let config_volume = config_volume.clone();

                        async move {
                            connection.run_cmd(certs_volume.command()).await?;
                            connection.run_cmd(config_volume.command()).await?;

                            Ok(())
                        }
                    })
                    .await?
            }
        }
        ContainerOp::Install => {
            let get_install_script = container_ops.get_install_script()?;
            let install_docker = container_ops.install_docker()?;

            controller
                .on_hosts(|connection| {
                    let get_install_script = get_install_script.clone();
                    let install_docker = install_docker.clone();
                    async move {
                        connection.run_cmd(get_install_script.command()).await?;
                        connection.run_cmd(install_docker.command()).await?;

                        Ok(())
                    }
                })
                .await?
        }
        ContainerOp::Build => {
            for image in container_ops.images {
                let build = image.build()?;

                controller
                    .on_hosts(|connection| {
                        let build = build.clone();
                        async move {
                            connection.run_cmd(build.command()).await?;

                            Ok(())
                        }
                    })
                    .await?
            }
        }
        ContainerOp::Start => {
            for image in container_ops.images {
                let start = image.start()?;

                controller
                    .on_hosts(|connection| {
                        let start = start.clone();
                        async move {
                            connection.run_cmd(start.command()).await?;

                            Ok(())
                        }
                    })
                    .await?
            }
        }
        ContainerOp::List => {
            let list_containers = container_ops.list_containers()?;

            controller
                .on_hosts(|connection| {
                    let list_containers = list_containers.command();

                    async move {
                        connection.run_cmd(list_containers).await?;

                        Ok(())
                    }
                })
                .await?
        }
        ContainerOp::Login => {
            let login = container_ops.login()?;

            controller
                .on_hosts(|connection| {
                    let login = login.clone();
                    async move {
                        connection.run_cmd(login.command()).await?;

                        Ok(())
                    }
                })
                .await?
        }
        ContainerOp::Pull => {
            for image in container_ops.images {
                let pull = image.pull()?;

                controller
                    .on_hosts(|connection| {
                        let pull = pull.clone();
                        async move {
                            connection.run_cmd(pull.command()).await?;

                            Ok(())
                        }
                    })
                    .await?
            }
        }
        ContainerOp::Restart => {
            for image in container_ops.images {
                let kill = image.kill_all()?;
                let pull = image.pull()?;
                let run = image.start()?;

                controller
                    .on_hosts(|connection| {
                        let kill = kill.clone();
                        let pull = pull.clone();
                        let run = run.clone();

                        async move {
                            connection.run_cmd(kill.command()).await?;
                            connection.run_cmd(pull.command()).await?;
                            connection.run_cmd(run.command()).await?;

                            Ok(())
                        }
                    })
                    .await?
            }
        }
        ContainerOp::Push => {}
    }

    Ok(())
}
