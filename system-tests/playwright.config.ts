import { defineConfig, devices } from "@playwright/test";

export default defineConfig({
  testDir: "./tests/cases",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: 0,
  workers: 1,
  reporter: "html",
  use: {
    baseURL: process.env.CONSOLE_URL || "https://console.localhost",
    ignoreHTTPSErrors: true,
    screenshot: "only-on-failure",
    video: "on",
    trace: "on",
  },
  projects: [
    {
      name: "desktop",
      use: {
        ...devices["Desktop Chrome"],
        viewport: { width: 1280, height: 720 },
      },
    },
    {
      name: "tablet",
      use: {
        ...devices["Desktop Chrome"],
        viewport: { width: 768, height: 1024 },
      },
    },
  ],
});
