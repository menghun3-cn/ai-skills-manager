import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import router from "./router";
import { i18n } from "./i18n";
import "./styles/main.css";

const app = createApp(App);

app.use(createPinia());
app.use(router);
app.use(i18n);

app.mount("#app");

// 禁用 F1 快捷键（防止打开帮助）
document.addEventListener("keydown", (e) => {
  if (e.key === "F1") {
    e.preventDefault();
  }
});
