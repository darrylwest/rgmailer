# rg mailer

```bash
                                   __ __             
.----.-----.______.--------.---.-.|__|  |.-----.----.
|   _|  _  |______|        |  _  ||  |  ||  -__|   _|
|__| |___  |      |__|__|__|___._||__|__||_____|__|  
     |_____|                                         
```

## Overview

Rust SMTP mailer utility using gmail or other SMTP services.  The application is a utility desiged 
to be called by other services, monitoring, backup, etc as a way to send error or other critical 
messages.  Messages are simple text and don't support attachments.  Links can be imbedded if additional
information is available to explain the error or problem.

## Use

1. create an email envelope/toml file with to, from, subject, body and optional process name
2. copy the email file to the queue folder (optional)
3. call rgmailer with the path to the email file
4. evaluate the response to ensure the mail was successfully sent

Example email envelope file

```toml

to: recipient@gmail.com
from: sender@gmail.com
subject: low disk space on machine 113...
body: available free disk space is now @ 2.65Mb.  please remove unused files as soon as possible.

```

## File Requirements

* logging config files for rotating file, console, and file+console (log4rs format)
* settings.toml - access to the SMTP service


###### 2023-11-03
