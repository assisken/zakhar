# Команды

## Как меня зовут

Бот изменяет nickname пользователя, вызвавшего данную команду, на случайный из перечня (либо генерируемый каким-либо образом).

```mermaid
sequenceDiagram
    actor U as Пользователь
    participant B as Бот
    participant D as Discord API

    U->>+B: Как меня зовут?

    B->>+D: Поменяй ему nickname
    D-->>-B: Готово

    B-->>-U: Тебя зовут: жаренная ватрушка
```

## Скажи что-нибудь

По запросу пользователя бот генерирует случайную фразу, используя цепи Маркова.

```mermaid
sequenceDiagram
    actor U as Пользователь
    participant B as Бот
    participant C as Цепи Маркова

    U->>B: Любое сообщение
    B->>C: Добавить слова
    C->>C: Обновить цепочку

    U->>+B: Скажи что-нибудь!
    B->>+C: Создай цепочку, пж
    C-->>-B: Цепочка
    B-->>-U: У Лукоморья дуб зелёный...
```

## Карусель

Пользователя, находящегося в голосовом канале, перекидывает по всем доступным для него каналам в случаном порядке. Прекращается либо по таймеру (возможный аргумент команды), либо по вводу команды-отмены.

```mermaid
sequenceDiagram
    actor U as Пользователь
    participant B as Бот
    participant D as Discord API

    Note over U: Находится в голосовом канале

    U->>+B: Хочу кататься!
    B-->>U: Понял тебя

    loop Карусель
    B->>+D: Перекинь его ещё в какой-нибудь канал
    D-->>-B: Готово
    end

    U->>B: Меня укачивает!
    B-->>-U: Здорово!
```

## Розыгрыш

Наугад (либо с помощью аргумента команды указывается конкретный пользователь) выбирается голосовой канал, куда подключается бот и начинает проигрывать "АТВИНТА (Extreme Bass Boost)" (или нечто подобное). Аудио дорожки могут выбираться наугад из банка. Длительность дорожки не должна быть слишком большой - в противном случае будет уже не смешно.

```mermaid
sequenceDiagram
    actor U as Пользователь
    participant B as Бот
    participant D as Discord API

    U->>B: Давай разыграем Александра!
    activate B
    B->>+D: Удали сообщение пользователя
    D-->>-B: Готово
    B->>+D: В каком голосовом канале находится Александр?
    D-->>-B: Вот в этом
    B->>+D: Подключаемся к нему и ставим вот этот трек
    D-->>-B: Готово
    deactivate B

    Note over B: По окончании проигрывания трека бот отключается из голосового канала
```

## Брак

Система брака. В качестве аргумента команды указывается партнёр (либо оба партнёра). Далее каждый из кандидатов должен выразить своё согласие (нажатием на кнопку, вводом ключевой фразы...). Бот хранит список пар.

```mermaid
sequenceDiagram
    actor U1 as Пользователь 1
    participant B as Бот
    actor U2 as Пользователь 2

    U1->>+B: Жениться бы мне на Пользователе 2...
    B-->>U1: Ты уверен?
    U1->>B: Да!
    B->>U2: Ты согласна?
    U2-->>B: Оф кокос!

    B->>-B: Запомнить пару

    Note over B: Бот объявляет в чате новую пару
```