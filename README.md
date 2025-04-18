# Rybalko — Lab 5 for PIS

## Вправа 6.1-6.2 🔗 Зовнішній сервіс

У цьому завданні використовується **публічне API ПриватБанку** для отримання актуального курсу валют:

📄 **Зовнішній API ПриватБанку:**
https://api.privatbank.ua/p24api/pubinfo?exchange&coursid=5

📄 **Локальний ендпоінт вебсервера:**
http://localhost:3000/rate

### 🖥️ Приклад відповіді:

```html
<h3>ccy: USD</h3>
<h3>base ccy: UAH</h3>
<h3>buy: 38.10</h3>
<h3>sale: 38.60</h3>
```

## 🛠️ Реалізація

Проєкт реалізовано з використанням:

- [Rust](https://www.rust-lang.org/)
- [Axum](https://docs.rs/axum) — асинхронний вебфреймворк
- [Reqwest](https://docs.rs/reqwest) — HTTP-клієнт
- [Serde](https://docs.rs/serde) — для парсингу JSON
- [Build HTML](https://docs.rs/build-html) — для генерації HTML-відповіді
- [Tracing](https://docs.rs/tracing) — для логування

## 📌 Сценарій використання

Сервер звертається до публічного API ПриватБанку, щоб отримати актуальний курс обміну валют (наприклад, USD → UAH) у реальному часі:

Отримані дані використовуються для формування HTML-відповіді, яку бачить користувач за запитом на `/rate`.

Ця інформація може бути використана системою згідно з логікою теми  
_**«Облік курсової різниці в касі»**_ — наприклад, для обліку валютних операцій або створення звітів.

## 🧪 Вправа 6.3 — HTTP-сервер з обробкою логіну Moodle

Реалізовано HTTP-сервер, який обробляє запит із логіном Moodle та повертає HTML-відповідь з особистими даними.

### 🔗 Маршрут:

_**GET /moodle/{login}**_
