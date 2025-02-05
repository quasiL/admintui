# AdminTUI

AdminTUI is a terminal user interface (TUI) app designed for system administration tasks. It provides an easy-to-use interface for managing various configurations and processes in a Linux-based environment.

## How to build

To build the application, use the following Docker Compose command:

```
docker compose -f docker/docker-compose.build.yml up --build
```

The build process utilizes Docker to ensure a consistent environment, especially due to differences in system libraries and dependencies across various machines. Specifically, the build process requires a specific version of glibc (GNU C Library) to ensure compatibility with the application's dependencies.

## Environment Configuration

All necessary environment variables are stored in the .env file.
