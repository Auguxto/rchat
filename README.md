# RChat - Chat em Rust com Tokio

RChat é um simples aplicativo de chat cliente-servidor implementado em Rust utilizando a biblioteca assíncrona Tokio. O projeto consiste em dois componentes principais: um servidor de chat e um cliente de chat.

## Estrutura do Projeto

```
├── chat-client/       # Cliente de chat
│   ├── Cargo.toml     # Configuração e dependências do cliente
│   └── src/
│       └── main.rs    # Implementação do cliente
└── chat-server/       # Servidor de chat
    ├── Cargo.toml     # Configuração e dependências do servidor
    └── src/
        └── main.rs    # Implementação do servidor
```

## Funcionalidades

- **Servidor de Chat**: Aceita múltiplas conexões de clientes e distribui mensagens entre eles
- **Cliente de Chat**: Conecta-se ao servidor, envia mensagens e recebe mensagens de outros clientes
- **Comunicação em Tempo Real**: Utiliza o modelo de programação assíncrona do Tokio para comunicação eficiente
- **Broadcast de Mensagens**: Mensagens enviadas por um cliente são distribuídas para todos os outros clientes conectados

## Requisitos

- [Rust](https://www.rust-lang.org/tools/install) (versão estável mais recente)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (geralmente instalado com Rust)

## Como Executar

### 1. Iniciar o Servidor

Primeiro, inicie o servidor de chat:

```bash
cd chat-server
cargo run
```

O servidor será iniciado e ficará escutando na porta 8080 do localhost (127.0.0.1).

### 2. Iniciar um ou mais Clientes

Em outro terminal, inicie um cliente de chat:

```bash
cd chat-client
cargo run
```

Você pode iniciar múltiplos clientes em diferentes terminais para simular uma conversa entre vários usuários.

### 3. Enviar Mensagens

Após iniciar um cliente, você pode digitar mensagens no terminal e pressionar Enter para enviá-las. As mensagens serão exibidas para todos os clientes conectados.

### 4. Encerrar o Cliente

Para encerrar um cliente, pressione `Ctrl+C` no terminal onde ele está sendo executado.

## Detalhes Técnicos

### Servidor

- Utiliza `TcpListener` do Tokio para aceitar conexões
- Implementa um sistema de broadcast usando `tokio::sync::broadcast`
- Gerencia múltiplas conexões simultâneas usando tarefas assíncronas (tasks)
- Cada cliente é tratado em uma task separada

### Cliente

- Conecta-se ao servidor usando `TcpStream` do Tokio
- Utiliza tasks separadas para:
  - Ler mensagens do servidor
  - Ler input do usuário e enviar ao servidor
- Implementa tratamento de sinal `Ctrl+C` para encerramento gracioso

## Limitações Atuais

- Não há autenticação de usuários
- Não há persistência de mensagens
- Interface limitada ao terminal
