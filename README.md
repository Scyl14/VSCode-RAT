# VSCode-RAT

This work is a rust implementation a relatively known threat seen in the wild:

This malware abuses the legitimate tunnel functionality implemented by Visual Studio Code. Granting user level access to a target host via the visual studio code IDE web interface.

The concept on witch the malware is based i fairly simple, first check the presence of visual studio code no the system.

If it's not already present download code.exe from the microsoft official website.

Then runs code.exe starting the actual tunnel and sends the machine authentication OTP and hostname to a remote server owned by the TA.

These two information are all the TA needs to initiate the tunnel.

Uses the OTP to log in to:

https://github.com/login/device

And then connects to te actual tunnel visiting:

https://vscode.dev/tunnel/HOSTNAME/C:

After that the TA can freely navigate the file system uploading and downloading files.

At the moment, since the malware use only legitimate traffic to VS code (just the first connetion to send the OTP and the hostname can be suspicious) is not detected by main EDR and AV tecnologies. (Obvusly post exploitation is still fully detectable)

TO DO:

1. Add persistence
