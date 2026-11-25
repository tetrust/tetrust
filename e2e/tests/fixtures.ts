import { test as base, expect, type Page } from '@playwright/test';

// 모든 테스트에서 페이지 오류(wasm 패닉 포함)가 없어야 한다.
export const test = base.extend<{ pageErrors: Error[] }>({
  pageErrors: [
    async ({ page }, use) => {
      const errors: Error[] = [];
      page.on('pageerror', (error) => errors.push(error));
      await use(errors);
      expect(errors, errors.map((e) => e.message).join('\n')).toEqual([]);
    },
    { auto: true },
  ],
});

export { expect };

export async function openSingle(page: Page) {
  await page.goto('/single');
  await expect(page.locator('#game-canvas')).toBeVisible();
}

export async function startGame(page: Page) {
  await page.getByRole('button', { name: 'Start (R)' }).click();
  await expect(page.locator('#pieces')).toHaveText('0');
}
