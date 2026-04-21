import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      redirect: "/skills",
    },
    {
      path: "/skills",
      name: "skills",
      component: () => import("@/views/SkillsPage.vue"),
    },
    {
      path: "/tools",
      name: "tools",
      component: () => import("@/views/ToolsPage.vue"),
    },
    {
      path: "/market",
      name: "market",
      component: () => import("@/views/MarketPage.vue"),
    },
    {
      path: "/settings",
      name: "settings",
      component: () => import("@/views/SettingsPage.vue"),
    },
  ],
});

export default router;
