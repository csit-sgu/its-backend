use poem_openapi::Tags;

#[derive(Tags)]
pub enum ApiTag {
    /// Проверка
    Index,
    /// Задачи
    Tasks,
    /// Авторизация
    Login,
    Transitions,
}
