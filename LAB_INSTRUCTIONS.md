# Lab 1: Running the Algonquin Pet Store Locally and on an Azure VM

Welcome to the first lab of the **CST8915 Full-stack Cloud-native Development**. In this lab, you will learn how to run the Algonquin Pet Store locally on your machine, and then deploy it to an Azure Virtual Machine. This hands-on exercise will teach you how to run a multi-service application and work with cloud deployment.

## Pre-Lab Instructions:
Before attending the lab, it is recommended to complete the following tasks:
- Read [Algonquin Pet Store README](README.md)
- Examine and understand the Algonquin Pet Store architecture.


## Important Note for Students:
Before you start the lab, **please read through the entire instructions carefully.** It is important to understand the flow and requirements fully before beginning the tasks. If you are working in a group, make sure all team members are familiar with the instructions and assigned responsibilities.

## Lab Objectives:
- Set up and run the application on your local machine.
- Set up and run the application on an Azure VM.

## Group Work Recommendation
We recommend that students work in **groups of 4** for this lab. This will help you practice teamwork, share the workload, and simulate real-world development environments. If you prefer to work individually, that's fine as well.
### Group Guidelines

- **Divide the tasks**: Each team member can be responsible for a particular service (e.g., order-service, product-service, store-front, RabbitMQ setup).
- **Deploy the app together**: If you're working in a group, you can deploy the full system to one Azure VM.


## Prerequisites:
- **Git**: Ensure you have Git installed on your machine.
- **Node.js**: You’ll need Node.js for the store-front and order-service.
- **Vue CLI**: You’ll need Vue CLI for the store-front.
- **Rust**: You’ll need Rust for the product-service.
- **Azure Account**: Make sure you have access to a Microsoft Azure account.
- **VS Code**: Install [Visual Studio Code](https://code.visualstudio.com/).
- **WSL (Windows Users)**: If you're using Windows, it is recommended to use **Windows Subsystem for Linux (WSL)** for your development environment.

**Note:** For step-by-step instructions on setting up Node.js, Vue CLI, and Rust, refer to the [Algonquin Pet Store README](README.md).

---

## Step 1: Running the Algonquin Pet Store Locally
### 1.1. Setting Up VS Code with WSL (for Windows Users)

For local development, you can use **VS Code**. We recommend that **Windows users** work with **WSL**. However, feel free to use whatever development environment you're comfortable with (Linux, macOS, etc.).
Follow the instruction here: [Developing with VS Code in WSL](https://code.visualstudio.com/docs/remote/wsl), to get started with developing in WSL.


The Visual Studio Code WSL extension lets you use the Windows Subsystem for Linux (WSL) as your full-time development environment right from VS Code. You can develop in a Linux-based environment, use Linux-specific toolchains and utilities, and run and debug your Linux-based applications all from the comfort of Windows.
### 1.2. Set up and run the application on your local machine.
- Open your terminal in VS Code or WSL.
- Clone the repository:
    ```bash
    git clone https://github.com/ramymohamed10/algonquin-pet-store.git
    cd algonquin-pet-store
- Follow the instructions in [Algonquin Pet Store README](README.md) to set up the app on your local machine.


## Step 2: Running the Algonquin Pet Store on an Azure VM
### 2.1. Setting Up an Azure Virtual Machine
- Go to the Azure portal.
- Click Create a Resource and select Virtual Machine.
- Choose the following configurations:
- Image: Choose Ubuntu 22.04 LTS.
- Size: Select an appropriate VM size (e.g., Standard B2s) for small-scale testing.
- Authentication: Use an SSH public key for secure access.

Once your VM is created, note down the public IP address for future use.

### 2.2. Configure the Network Security Group (NSG)
After creating the VM, you need to configure the **Network Security Group (NSG)** attached to the VM to allow access to specific ports.

- Go to the **Networking** section of your VM in the Azure portal.
- Ensure that the **Inbound port rules** and **Outbound port rules** match the following configuration:

#### Inbound Port Rules:

| Priority | Name                     | Port  | Protocol | Source | Destination | Action |
|----------|--------------------------|-------|----------|--------|-------------|--------|
| 300      | SSH                       | 22    | TCP      | Any    | Any         | Allow  |
| 320      | HTTP                      | 80    | TCP      | Any    | Any         | Allow  |
| 340      | HTTPS                     | 443   | TCP      | Any    | Any         | Allow  |
| 350      | AllowAnyCustom8080Inbound | 8080  | Any      | Any    | Any         | Allow  |
| 360      | AllowAnyCustom15672Inbound| 15672 | Any      | Any    | Any         | Allow  |
| 370      | AllowAnyCustom3000Inbound | 3000  | Any      | Any    | Any         | Allow  |
| 380      | AllowAnyCustom3030Inbound | 3030  | Any      | Any    | Any         | Allow  |
| 65000    | AllowVnetInBound          | Any   | Any      | VirtualNetwork | VirtualNetwork | Allow |
| 65001    | AllowAzureLoadBalancerInBound | Any   | Any      | Any    | AzureLoadBalancer | Allow |
| 65500    | DenyAllInBound            | Any   | Any      | Any    | Any         | Deny   |

#### Outbound Port Rules:

| Priority | Name               | Port  | Protocol | Source       | Destination | Action |
|----------|--------------------|-------|----------|--------------|-------------|--------|
| 65000    | AllowVnetOutBound   | Any   | Any      | VirtualNetwork | VirtualNetwork | Allow  |
| 65001    | AllowInternetOutBound | Any   | Any      | Any          | Internet    | Allow  |
| 65500    | DenyAllOutBound     | Any   | Any      | Any          | Any         | Deny   |

Ensure that the security group rules are set up correctly so that necessary services (like SSH, HTTP, and custom application ports) can be accessed.

### 2.3. Setting Up VS Code for Remote Development on Azure VM
Instead of using a regular SSH terminal, you can use VS Code to remotely access the VM, making it easier to work on files and manage services.

- In VS Code, open the Extensions view (Ctrl + Shift + X). Search for and install the Remote - SSH extension.
- Open the command palette in VS Code (Ctrl + Shift + P). Type Remote-SSH: Connect to Host and select it.
- Add the SSH connection details using the public IP of your Azure VM. VS Code will connect to your VM and open a remote session.
- You can now work on files, open terminals, and install dependencies directly within VS Code.

### 2.4. Install Dependencies on the VM
Once connected to the Azure VM through VS Code:
- Open a new terminal in VS Code (Ctrl + Shift + P, then type Terminal: Create New Terminal).
- Clone the Repository on the VM
- Follow the instructions in [Algonquin Pet Store README](README.md) to set up the app on your VM.
- Once all services are running, you can access the app by opening the store-front from your local machine:
    ```bash
    http://<your-vm-ip>:8080
## Lab Task: Updating API Endpoints
As you prepare to run your Algonquin Pet Store application on an Azure Virtual Machine (VM), you might run into an issue with the front-end Vue.js application failing to fetch data from the backend services (Order and Product services). The current implementation works locally because the APIs are hosted at localhost, but when deployed to an Azure VM, localhost in the browser context will no longer point to the services running on the server.

**Figure out how to resolve this issue.**
## Conclusion
In this lab, you have:
- Set up and run the application locally using VS Code (with WSL for Windows users).
- Deployed the application to an Azure VM and accessed it using VS Code remote development.