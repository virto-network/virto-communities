use dioxus::prelude::*;

#[derive(Clone)]
pub struct Theme {
    pub background: String,
    pub text_1: String,
    pub text_2: String,
    pub text_3: String,
    pub text_4: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: String::from("var(--olive-100)"),
            text_1: String::from("var(--white)"),
            text_2: String::from("#FFFFFF7F"),
            text_3: String::from("var(--green-dark)"),
            text_4: String::from("#FFFFFF4C"),
        }
    }
}

pub fn use_theme() -> UsethemeState {
    let theme = consume_context::<Signal<Theme>>();

    use_hook(|| UsethemeState { inner: theme })
}

#[derive(Clone, Copy)]
pub struct UsethemeState {
    inner: Signal<Theme>,
}

impl UsethemeState {
    pub fn get(&self) -> Theme {
        self.inner.read().clone()
    }

    pub fn set(&mut self, theme: Theme) {
        let mut inner = self.inner.write();
        *inner = theme;
    }

    pub fn set_background(&mut self, background: String) {
        let mut inner = self.inner.write();
        inner.background = background;
    }

    pub fn set_text_1(&mut self, text_1: String) {
        let mut inner = self.inner.write();
        inner.text_1 = text_1;
    }

    pub fn set_text_2(&mut self, text_2: String) {
        let mut inner = self.inner.write();
        inner.text_2 = text_2;
    }

    pub fn set_text_3(&mut self, text_3: String) {
        let mut inner = self.inner.write();
        inner.text_3 = text_3;
    }

    pub fn set_text_4(&mut self, text_4: String) {
        let mut inner = self.inner.write();
        inner.text_4 = text_4;
    }

    pub fn get_style(self) -> String {
        let inner = self.inner.read();

        format!(
            r#"
                --background: {};
                --text-1: {};
                --text-2: {};
                --text-3: {};
                --text-4: {};
            "#,
            inner.background, inner.text_1, inner.text_2, inner.text_3, inner.text_4
        )
    }

    pub fn default(&mut self) {
        self.set(Theme::default())
    }
}
