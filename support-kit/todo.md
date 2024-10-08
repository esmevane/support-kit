# todos

- environment
- init color
- test from config file sources
  - file order
    - cli determines config name
      - use config if present
      - use name to create config if not
    - precedence
      - home config file
      - base file config (admerge overwrites home config)
      - env tagged file (admerge overwrites base config)
  - precedence
    - yaml
    - json (admerge overwrites yaml)
    - toml (admerge overwrites json)
    - cli (admerge overwrites toml)
    - env (admerge overwrites cli)
