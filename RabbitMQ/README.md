# RabbitMQ 

RabbitMQ is an open-source message broker that acts as an intermediary for messaging between different systems or services. In a microservices architecture, RabbitMQ allows different services to communicate with each other asynchronously and reliably through message queues. 

## Why RabbitMQ?
In microservice architectures, RabbitMQ is used to decouple services, meaning services do not directly communicate with each other. Instead, they send and receive messages through RabbitMQ. This allows each service to function independently, making the application more scalable and easier to maintain. RabbitMQ is built on the AMQP (Advanced Message Queuing Protocol), which allows for robust, reliable messaging.

## Core Concepts
To understand RabbitMQ, it's important to be familiar with a few core concepts:

- **Producer:** A producer is an application or service that sends messages to RabbitMQ. In our application, the Order Service acts as a producer when it sends an order to be processed.
- **Consumer:** A consumer is a service that receives and processes messages from RabbitMQ. For example, another service could act as a consumer by receiving order-related messages from RabbitMQ for further processing or shipping.
- **Queue:** A queue is where RabbitMQ stores messages before they are processed by consumers. Queues provide a way to buffer messages, so if one service sends a message faster than the other can process it, RabbitMQ will hold onto the message in the queue.

## Requirements

- RabbitMQ installed locally

RabbitMQ can be installed on various platforms, and it is necessary to have it running locally for this application to work. The management interface and client communication are handled via HTTP API and AMQP protocols.

## Setup Instructions

1. Follow the official RabbitMQ installation guide: https://www.rabbitmq.com/docs/install-debian#apt-quick-start-cloudsmith

   If you are using a different operating system, check RabbitMQ Official Installation Docs for other options.

2. Once Installed, start RabbitMQ:

   After installation, you need to start the RabbitMQ server so it can begin processing messages.

   ```bash
   sudo systemctl start rabbitmq-server  
3. Enable the RabbitMQ management plugin:

   RabbitMQ comes with a handy web-based management interface that allows you to monitor queues, exchanges, connections, and more. You'll need to enable it with the following command:

   ```bash
   sudo rabbitmq-plugins enable rabbitmq_management
4. Restart RabbitMQ

   To make sure the management plugin is fully activated, restart the RabbitMQ server:

   ```bash
   sudo systemctl restart rabbitmq-server
5. Create a New User for Remote Access, assign administrator privileges to the new user, and set permissions for the new user:

   For security purposes, it is recommended that you create a new user for managing RabbitMQ. The following commands will create a new user and assign administrative privileges:

   ```bash
   sudo rabbitmqctl add_user newuser newpassword
   sudo rabbitmqctl set_user_tags newuser administrator
   sudo rabbitmqctl set_permissions -p / newuser ".*" ".*" ".*"
6. Access the management console via http://localhost:15672:
   
    15672 is the port used for RabbitMQ's Management Web UI and HTTP API, which you can use to monitor queues, exchanges, and other RabbitMQ metrics. 
    
    The default RabbitMQ guest user can only access the RabbitMQ Management UI from localhost (127.0.0.1). This is a security restriction to prevent remote access using the default credentials.

    Here, you can log in using the default credentials:

   ```bash
   Default username: guest, Default password: guest
7. AMQP Port (Application Messaging Protocol):

    5672 is the default port used by RabbitMQ for communication with AMQP clients (such as producers and consumers). If you're connecting your application to RabbitMQ using a standard AMQP library, this is typically the port being used.
