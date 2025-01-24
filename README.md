# VSCode-RAT

This malware leverages the legitimate tunneling functionality implemented in Visual Studio Code, specifically its integration with GitHub for secure remote access. By abusing this feature, it provides unprivileged attackers with seamless access to a compromised host via the Visual Studio Code IDE web interface.

The malware operates on a straightforward yet effective concept:

Initial Check: It first verifies whether Visual Studio Code is installed on the target machine.
Automatic Installation: If Visual Studio Code is not already present, the malware silently downloads the latest version of code.exe directly from the official Microsoft website to ensure it remains inconspicuous.
Tunnel Initialization: The malware executes code.exe to start the tunnel, which automatically generates a unique machine authentication OTP. Alongside the OTP, the machine's hostname is collected and transmitted to a remote server controlled by the threat actor (TA).
These two pieces of information – the OTP and hostname – are sufficient for the TA to hijack the tunneling session without requiring any direct access to the victim's machine.

## Establishing the Connection

The TA performs the following steps:

Uses the OTP to authenticate at:

https://github.com/login/device

This step links the tunnel to the TA's GitHub account, bypassing any need for further interaction with the victim's machine.

Accesses the active tunnel by navigating to:

https://vscode.dev/tunnel/HOSTNAME/C:

Once connected, the TA can browse the victim's file system, upload malicious payloads, and exfiltrate sensitive data. The use of legitimate GitHub and Visual Studio Code infrastructure ensures the malicious traffic blends seamlessly with normal activity, making detection significantly harder.

## Detection Challenges

Since the malware relies entirely on legitimate Visual Studio Code traffic, its activity is nearly invisible to most endpoint detection and response (EDR) systems and antivirus (AV) solutions. The only suspicious behavior might be the initial connection, where the OTP and hostname are transmitted to the TA’s server. However, this step is brief and easily overlooked.

That said, any post-exploitation activities (e.g., uploading or downloading large files, executing scripts, etc.) are detectable by security tools monitoring file system or network behavior.

TO DO:
Add persistance.
