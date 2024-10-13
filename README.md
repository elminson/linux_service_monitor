# README

## Configuration File (`config.json`)

The `config.json` file is used to configure the service monitoring application. Below is an example of the `config.json` file and an explanation of each field:

```json
{
  "debug": true,
  "sleep": 2,
  "services": [
    {
      "name": "nginx",
      "active_text": "is running",
      "inactive_text": "is not running"
    },
    {
      "name": "mariadb",
      "active_text": "Uptime:",
      "inactive_text": "MariaDB is stopped"
    },
    {
      "name": "postgresql",
      "active_text": "online",
      "inactive_text": "down"
    },
    {
      "name": "fake_service",
      "active_text": "online",
      "inactive_text": "down"
    }
  ]
}
```

### Fields

- **debug**: A boolean value (`true` or `false`). When set to `true`, the application will print additional debug information to the console.
- **sleep**: An integer value representing the number of seconds the application will wait before checking the services' status again.
- **services**: An array of service configurations. Each service configuration contains the following fields:
    - **name**: The name of the service to be monitored.
    - **active_text**: A string that should be present in the service's status output when the service is active.
    - **inactive_text**: A string that should be present in the service's status output when the service is inactive.

### Filling `active_text` and `inactive_text`

To correctly fill the `active_text` and `inactive_text` fields for each service, follow these steps:

1. **Identify the Service**: Determine the name of the service you want to monitor. This is typically the name you would use with the `service` command (e.g., `nginx`, `mariadb`, `postgresql`).

2. **Check Service Status**: Run the command `service <service_name> status` in your terminal to check the status of the service. For example:
   ```sh
   service nginx status
   ```

3. **Determine Active Text**: Look at the output of the status command when the service is running. Identify a unique string that indicates the service is active. For example, for `nginx`, you might see:
   ```
   nginx is running
   ```
   In this case, `active_text` would be `is running`.

4. **Determine Inactive Text**: Look at the output of the status command when the service is stopped. Identify a unique string that indicates the service is inactive. For example, for `nginx`, you might see:
   ```
   nginx is not running
   ```
   In this case, `inactive_text` would be `is not running`.

5. **Update `config.json`**: Fill in the `active_text` and `inactive_text` fields for each service in the `config.json` file based on the strings you identified.

By following these steps, you can ensure that the application correctly identifies the status of each service based on the output of the `service` command.