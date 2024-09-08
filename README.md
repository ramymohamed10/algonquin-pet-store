# Algonquin Pet Store

Welcome to the **Algonquin Pet Store** application. This repository contains three core components that work together to create a cloud-native microservices architecture using Vue.js, Node.js, Rust, and RabbitMQ.

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Setup and Prerequisites](#setup-and-prerequisites)
3. [Running the Services](#running-the-services)
   - [Store Front](#store-front)
   - [Order Service](#order-service)
   - [Product Service](#product-service)
4. [Running RabbitMQ](#running-rabbitmq)
5. [Testing the Services](#testing-the-services)
6. [Common Issues and Troubleshooting](#common-issues-and-troubleshooting)

---

## Architecture Overview

![App Architecture](./Docs/app-architecture.png)

This pet store application is built using a microservices architecture with the following components:

- **Store Front (Vue.js)**: A front-end application where customers can browse and order products.
- **Order Service (Node.js)**: Manages customer orders and interacts with RabbitMQ for message queuing.
- **Product Service (Rust)**: Manages product details and inventory.
- **RabbitMQ**: Used to queue orders for processing.

## Setup and Prerequisites

Before starting, ensure you have the following installed:

- [Node.js (v20.x)](https://nodejs.org/)
- [Rust](https://www.rust-lang.org/tools/install)
- [RabbitMQ](https://www.rabbitmq.com/download.html) (You can use the provided `install_rabbitmq.sh` script)

