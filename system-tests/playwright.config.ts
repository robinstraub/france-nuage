import { defineConfig, devices } from "@playwright/test";

export default defineConfig({
  testDir: "./tests/cases",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: 0,
  workers: 1,
  reporter: "html",
  use: {
    baseURL: process.env.CONSOLE_URL || "http://localhost:5173",
    ignoreHTTPSErrors: true,
    screenshot: "only-on-failure",
    video: "on",
    trace: "on",
  },
  projects: [
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"] },
    },
  ],
});
