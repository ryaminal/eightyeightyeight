# Specification: Operational Integration

## 1. Systemd Service

- **Goal:** Create a `systemd` service file to manage the `eightyeightyeight` application as a background service.
- **Requirements:**
  - The service should be ableto start the `record` command.
  - The service should automatically restart on failure.
  - The service should be configurable via an environment file.
  - The service definition should be placed in a file named `eightyeightyeight.service`.

## 2. Disk Space Management

- **Goal:** Prevent the application from filling up the disk.
- **Requirements:**
  - Add a configuration option `min_disk_space_mb` to `config.toml`.
  - Before starting to record, check if the available disk space on the output file's partition is above this threshold.
  - During recording, periodically check the available disk space.
  - If the disk space falls below the threshold, gracefully stop the pipeline and exit with a clear error message.
