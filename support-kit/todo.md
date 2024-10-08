# todos

- SupportControl inits with a config and has setup / teardown / execute
  - might make sense to start doing traits here so it can get tested?
- args don't go into figment directly so why serde them
- logging
  - from 1-or-many to struct
  - move verbosity up
  - add color support
- service
  - "com.{app-name}.app" as default?
- test needs
  - setting color
    - cli
    - config
    - defaults
