service = app_name + '.service'

subdir('icons')

config = configuration_data()
config.set('APP_NAME', app_name)
config.set('APP_ID', app_id)
config.set('ICON', app_id)
config.set('BINARY', binary)
config.set('PROJECT_GIT', git_url)

# ---------- Service File --------- #

if install_systemdunitdir
  configure_file(
    input: 'app.service.in',
    output: service,
    configuration: config,
    install: true,
    install_dir: systemd_systemdsystemunitdir,
  )
endif
