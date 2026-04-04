# BSH_API

Documentação da API BSH_API.

## Executando a API

1. Entre no diretório do projeto.
2. Execute o comando:

    cargo run

Por padrão, a API será executada em `0.0.0.0` na porta `9090`.

### Alterando IP e porta

Você pode definir o IP e a porta com os argumentos:

    cargo run -- --ip 127.0.0.1 --port 5050

Após a execução, a API já estará pronta para ser consumida.

## Arquitetura da API

A API possui rotas públicas e privadas.

### Rotas privadas

| Método | Rota | Descrição |
|--------|------|-----------|
| GET | `/api/user/<id>` | Retorna informações do usuário |
| GET | `/api/users` | Lista todos os usuários |
| GET | `/api/posts` | Lista todos os posts |
| DELETE | `/api/user/<id>` | Deleta usuário |
| POST | `/api/update` | Atualiza informações do usuário |
| POST | `/api/posts` | Envia um novo post |

### Rotas públicas

| Método | Rota | Descrição |
|--------|------|-----------|
| GET | `/` | Informa que a API está no ar |
| POST | `/api/user` | Cria um usuário |
| POST | `/api/login` | Faz login em uma conta |
| POST | `/api/refresh` | Recebe refresh token e retorna access token |

Cada rota possui um protocolo de envio de dados, explicado nas próximas seções.

## Autenticação

Para verificar se um usuário está autorizado a acessar uma rota privada, a API utiliza JWT, que carrega o id do usuário autenticado.

Ao fazer login, a API retorna:

- `access token`
- `refresh token`

### Access token

O access token permite o acesso aos endpoints privados, dependendo do nível de acesso do usuário.

### Refresh token

O refresh token serve para obter um novo access token após o atual expirar.  
O usuário deve enviar o token para `/api/refresh` e, se ele for válido, a API retornará um novo access token.

### Header de autorização

O JWT deve ser enviado no header `Authorization` com o prefixo `Bearer`:

    Authorization: Bearer token

Em todas as rotas privadas é obrigatório o uso de um token válido.

A validação de usuários admin é feita no banco.

## Utilizando as rotas

### Rotas privadas

#### GET `/api/user/<id>`

Retorna informações do usuário.

Exemplo:

    GET /api/user/1

#### DELETE `/api/user/<id>`

Deleta um usuário.

Exemplo:

    DELETE /api/user/1

#### POST `/api/update`

Atualiza informações do usuário.

Exemplo de JSON:

    {
        "id": 10,
        "nickname": "novo_nick_name",
        "nickcolor": "#b34f5f" -> Cor do nick em hexadecimal
    }

#### GET `/api/users`

Lista todos os usuários.

Exemplo:

    GET /api/users

#### GET `/api/posts`

Lista todos os posts.

Exemplo:

    GET /api/posts

#### POST `/api/posts`

Envia um novo post.

Exemplo de JSON:

    {
      "content": "mensagem a ser enviada"
    }

### Rotas públicas

#### POST `/api/user`

Cria um usuário.

Exemplo de JSON:

    {
      "role": "user",
      "nickname": "usuario1",
      "username": "nome_de_usu",
      "password": "123456"
    }

> Aqui o campo `role` define o nível do usuário, podendo ser `user` ou `admin`.

#### POST `/api/refresh`

Recebe refresh token e retorna access token.

Exemplo de JSON:

    {
      "token": "tokenaqui"
    }

#### POST `/api/login`

Faz login em uma conta.

Exemplo de JSON:

    {
      "username": "nome_de_usu",
      "password": "123456"
    }

Caso o login seja bem-sucedido, a API retornará o access token e o refresh token.