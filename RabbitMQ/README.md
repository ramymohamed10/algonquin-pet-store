# RabbitMQ 

RabbitMQ is used as the message broker for this application. It handles communication between services.

## Requirements

- RabbitMQ installed locally

## Setup Instructions

1. Follow the official RabbitMQ installation guide: https://www.rabbitmq.com/docs/download
2. Once Installed, start RabbitMQ:
   ```bash
   sudo systemctl start rabbitmq-server
3. Access the management console via http://localhost:15672:

    15672 is the port used for RabbitMQ's Management Web UI and HTTP API, which you can use to monitor queues, exchanges, and other RabbitMQ metrics.
   ```bash
   username: guest, password: guest
4. AMQP Port (Application Messaging Protocol):

    5672 is the default port used by RabbitMQ for communication with AMQP clients (such as producers and consumers). If you're connecting your application to RabbitMQ using a standard AMQP library, this is typically the port being used.
