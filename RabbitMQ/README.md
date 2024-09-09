# RabbitMQ 

RabbitMQ is used as the message broker for this application. It handles communication between services.

## Requirements

- RabbitMQ installed locally

## Setup Instructions

1. Follow the official RabbitMQ installation guide: https://www.rabbitmq.com/docs/install-debian#apt-quick-start-cloudsmith
2. Once Installed, start RabbitMQ:
   ```bash
   sudo systemctl start rabbitmq-server  
3. Enable the RabbitMQ management plugin:
   ```bash
   sudo rabbitmq-plugins enable rabbitmq_management
4. Restart RabbitMQ
   ```bash
   sudo systemctl restart rabbitmq-server
5. Create a New User for Remote Access, assign administrator privileges to the new user, and set permissions for the new user:
   ```bash
   sudo rabbitmqctl add_user newuser newpassword
   sudo rabbitmqctl set_user_tags newuser administrator
   sudo rabbitmqctl set_permissions -p / newuser ".*" ".*" ".*"
6. Access the management console via http://localhost:15672:

    15672 is the port used for RabbitMQ's Management Web UI and HTTP API, which you can use to monitor queues, exchanges, and other RabbitMQ metrics. 
    
    The default RabbitMQ guest user can only access the RabbitMQ Management UI from localhost (127.0.0.1). This is a security restriction to prevent remote access using the default credentials.
   ```bash
   Default username: guest, Default password: guest
7. AMQP Port (Application Messaging Protocol):

    5672 is the default port used by RabbitMQ for communication with AMQP clients (such as producers and consumers). If you're connecting your application to RabbitMQ using a standard AMQP library, this is typically the port being used.
