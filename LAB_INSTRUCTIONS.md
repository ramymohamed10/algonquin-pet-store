# Lab 1: Running the Algonquin Pet Store Locally and on an Azure VM

Welcome to the first lab of the **CST8915 Full-stack Cloud-native Development**. In this lab, you will learn how to run the app locally on your machine, and then deploy it to an Azure Virtual Machine.

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
- Size: Select an appropriate VM size (e.g., B1s) for small-scale testing.
- Authentication: Use an SSH public key for secure access.

Once your VM is created, note down the public IP address for future use.
### 2.2. Setting Up VS Code for Remote Development on Azure VM
Instead of using a regular SSH terminal, you can use VS Code to remotely access the VM, making it easier to work on files and manage services.

- In VS Code, open the Extensions view (Ctrl + Shift + X). Search for and install the Remote - SSH extension.
- Open the command palette in VS Code (Ctrl + Shift + P). Type Remote-SSH: Connect to Host and select it.
- Add the SSH connection details using the public IP of your Azure VM. VS Code will connect to your VM and open a remote session.
- You can now work on files, open terminals, and install dependencies directly within VS Code.

### 2.3. Install Dependencies on the VM
Once connected to the Azure VM through VS Code:
- Open a new terminal in VS Code (Ctrl + Shift + P, then type Terminal: Create New Terminal).
- Install Node.js, Vue cli, npm, Rust and RabbitMQ on the VM and any needed dependencies.
- Clone the Repository on the VM
- Follow the instructions in [Algonquin Pet Store README](README.md) to set up the app on your local machine.
- Once all services are running, you can access the app by opening the store-front from your local machine:
    ```bash
    http://<your-vm-ip>:8080

## Conclusion
In this lab, you have:
- Set up and run the application locally using VS Code (with WSL for Windows users).
- Deployed the application to an Azure VM and accessed it using VS Code remote development.