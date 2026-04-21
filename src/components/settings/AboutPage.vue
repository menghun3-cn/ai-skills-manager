<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { Github, Bug, MessageSquare, Heart, Sparkles, Code2, Zap, Shield } from "lucide-vue-next";

const { t } = useI18n();

const techStack = [
  { name: "Tauri 2.0", icon: Shield, color: "lavender" },
  { name: "Vue 3", icon: Code2, color: "mint" },
  { name: "TypeScript", icon: Zap, color: "honey" },
  { name: "TailwindCSS", icon: Sparkles, color: "peach" },
];

const links = [
  { id: "github", label: "GitHub", icon: Github, url: "https://github.com" },
  { id: "issues", label: t("settings.reportIssue"), icon: Bug, url: "https://github.com/issues" },
  { id: "discussions", label: t("settings.vote"), icon: MessageSquare, url: "https://github.com/discussions" },
];
</script>

<template>
  <div class="about-container">
    <!-- Logo 区域 -->
    <div class="logo-section">
      <div class="logo-wrapper">
        <div class="logo-glow"></div>
        <div class="logo-icon">
          <Sparkles class="logo-svg" :stroke-width="1.5" />
        </div>
      </div>
      <h1 class="app-name">AI Skills Manager</h1>
      <div class="version-badge">
        <span class="version-text">v1.0.0</span>
      </div>
      <p class="app-slogan">
        {{ t("settings.aboutSlogan") }}
      </p>
    </div>

    <!-- 功能卡片 -->
    <div class="feature-card">
      <div class="feature-icon">
        <Heart class="heart-icon" :stroke-width="2" />
      </div>
      <p class="feature-text">
        {{ t("settings.aboutDesc") }}
      </p>
    </div>

    <!-- 链接按钮 -->
    <div class="links-section">
      <a
        v-for="link in links"
        :key="link.id"
        :href="link.url"
        target="_blank"
        class="link-btn"
      >
        <component :is="link.icon" class="link-icon" :stroke-width="1.5" />
        <span>{{ link.label }}</span>
      </a>
    </div>

    <!-- 技术栈 -->
    <div class="tech-section">
      <h3 class="tech-title">{{ t("settings.techStack") }}</h3>
      <div class="tech-grid">
        <div
          v-for="tech in techStack"
          :key="tech.name"
          class="tech-item"
          :class="`tech-${tech.color}`"
        >
          <component :is="tech.icon" class="tech-icon" :stroke-width="1.5" />
          <span>{{ tech.name }}</span>
        </div>
      </div>
    </div>

    <!-- 版权信息 -->
    <div class="footer">
      <p class="copyright">
        {{ t("settings.copyright") }}
      </p>
    </div>
  </div>
</template>

<style scoped>
.about-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-xl);
  max-width: 500px;
  margin: 0 auto;
  padding: var(--spacing-lg);
}

/* Logo 区域 */
.logo-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  gap: var(--spacing-md);
}

.logo-wrapper {
  position: relative;
  width: 100px;
  height: 100px;
}

.logo-glow {
  position: absolute;
  inset: -10px;
  background: linear-gradient(135deg, rgba(255, 155, 173, 0.3), rgba(184, 169, 232, 0.3));
  border-radius: var(--radius-full);
  filter: blur(20px);
  animation: glowPulse 3s ease-in-out infinite;
}

@keyframes glowPulse {
  0%, 100% { opacity: 0.6; transform: scale(1); }
  50% { opacity: 1; transform: scale(1.05); }
}

.logo-icon {
  position: relative;
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--peach-300), var(--lavender-400));
  border-radius: var(--radius-full);
  box-shadow: var(--shadow-clay);
  color: white;
}

.logo-svg {
  width: 50px;
  height: 50px;
  animation: sparkle 2s ease-in-out infinite;
}

@keyframes sparkle {
  0%, 100% { transform: scale(1) rotate(0deg); }
  50% { transform: scale(1.1) rotate(5deg); }
}

.app-name {
  font-size: var(--text-2xl);
  font-weight: 700;
  color: var(--color-text-primary);
  margin: 0;
  font-family: var(--font-heading);
  background: linear-gradient(135deg, var(--peach-400), var(--lavender-400));
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.version-badge {
  padding: var(--spacing-xs) var(--spacing-md);
  background: var(--color-surface-subtle);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-full);
}

.version-text {
  font-size: var(--text-sm);
  font-weight: 600;
  color: var(--color-text-secondary);
  font-family: var(--font-mono);
}

.app-slogan {
  font-size: var(--text-base);
  color: var(--color-text-secondary);
  margin: 0;
  line-height: 1.6;
}

/* 功能卡片 */
.feature-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-md);
  padding: var(--spacing-xl);
  background: linear-gradient(135deg, var(--peach-50), var(--lavender-50));
  border: 2px solid var(--peach-200);
  border-radius: var(--radius-xl);
  text-align: center;
  width: 100%;
  position: relative;
  overflow: hidden;
}

.feature-card::before {
  content: '';
  position: absolute;
  inset: 0;
  background: linear-gradient(135deg, rgba(255, 155, 173, 0.1), rgba(184, 169, 232, 0.1));
  opacity: 0.5;
}

.feature-icon {
  position: relative;
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--peach-300), var(--peach-400));
  border-radius: var(--radius-full);
  box-shadow: var(--shadow-clay);
  animation: heartBeat 1.5s ease-in-out infinite;
}

@keyframes heartBeat {
  0%, 100% { transform: scale(1); }
  14% { transform: scale(1.1); }
  28% { transform: scale(1); }
  42% { transform: scale(1.1); }
  70% { transform: scale(1); }
}

.heart-icon {
  width: 24px;
  height: 24px;
  color: white;
}

.feature-text {
  position: relative;
  font-size: var(--text-base);
  color: var(--color-text-primary);
  margin: 0;
  line-height: 1.6;
  font-weight: 500;
}

/* 链接按钮 */
.links-section {
  display: flex;
  gap: var(--spacing-md);
  flex-wrap: wrap;
  justify-content: center;
  width: 100%;
}

.link-btn {
  display: inline-flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm) var(--spacing-lg);
  background: var(--color-surface);
  border: 2px solid var(--color-border);
  border-radius: var(--radius-lg);
  color: var(--color-text-secondary);
  font-size: var(--text-sm);
  font-weight: 500;
  text-decoration: none;
  transition: all var(--transition-fast) var(--ease-out);
  font-family: var(--font-body);
}

.link-btn:hover {
  background: var(--peach-50);
  border-color: var(--peach-200);
  color: var(--peach-500);
  transform: translateY(-2px);
  box-shadow: var(--shadow-soft);
}

.link-icon {
  width: 18px;
  height: 18px;
}

/* 技术栈 */
.tech-section {
  width: 100%;
  text-align: center;
}

.tech-title {
  font-size: var(--text-sm);
  font-weight: 600;
  color: var(--color-text-secondary);
  margin-bottom: var(--spacing-md);
  text-transform: uppercase;
  letter-spacing: 1px;
}

.tech-grid {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: var(--spacing-sm);
}

.tech-item {
  display: inline-flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-xs) var(--spacing-md);
  border-radius: var(--radius-full);
  font-size: var(--text-xs);
  font-weight: 600;
  transition: all var(--transition-fast) var(--ease-out);
}

.tech-peach {
  background: linear-gradient(135deg, var(--peach-100), var(--peach-50));
  color: var(--peach-500);
  border: 1px solid var(--peach-200);
}

.tech-lavender {
  background: linear-gradient(135deg, var(--lavender-100), var(--lavender-50));
  color: var(--lavender-500);
  border: 1px solid var(--lavender-200);
}

.tech-mint {
  background: linear-gradient(135deg, rgba(125, 212, 168, 0.2), rgba(125, 212, 168, 0.1));
  color: #4A9B7A;
  border: 1px solid rgba(125, 212, 168, 0.4);
}

.tech-honey {
  background: linear-gradient(135deg, rgba(255, 214, 102, 0.2), rgba(255, 214, 102, 0.1));
  color: #D4A030;
  border: 1px solid rgba(255, 214, 102, 0.4);
}

.tech-icon {
  width: 14px;
  height: 14px;
}

/* 版权信息 */
.footer {
  margin-top: var(--spacing-lg);
  padding-top: var(--spacing-lg);
  border-top: 1px solid var(--color-border-subtle);
  width: 100%;
  text-align: center;
}

.copyright {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
  margin: 0;
}
</style>
