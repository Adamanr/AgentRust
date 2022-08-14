<h1>AgentRust</h1>

<b>Ru</b>
<i>Агент - программа разработанная на языке Rust для камер под архитектуры ARM/Mipsel</i>
Функционал:
1. Агент берёт серийный номер из конфигурации камеры и отправляет его в базу данных
2. При получении номера камеры сервер отправляет данные на <a href="https://github.com/Adamanr/WireGuard_keygen">WireGuard-KeyGen</a>
3. WireGuard-KeyGen отправляет данные обратно на сервер
4. Агент получает ответ от сервера в виде данных пришедших после запроса сервера на WireGuard-KeyGen
5. Запускает VPN через команду с аргументами полученными в ответе от сервера

<b>En</b>
<i>Agent is a program developed on Rust language for cameras under ARM/Mips architecture Functional</i>
Functional:
1. The agent takes the serial number from the camera configuration and sends it to the database
2. When receiving the camera number, the server sends data to <a href="https://github.com/Adamanr/WireGuard_keygen">WireGuard-KeyGen</a>
3. WireGuard-KeyGen sends data back to the server
4. The agent receives a response from the server in the form of data received after the server request to WireGuard-KeyGen
5. Starts the VPN via the command with the arguments received in the response from the server
