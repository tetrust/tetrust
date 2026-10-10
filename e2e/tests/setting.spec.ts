import { test, expect } from './fixtures';

test('핸들링 설정은 새로고침해도 유지된다', async ({ page }) => {
  await page.goto('/setting');
  const inputs = page.locator('input[type=number]');
  await expect(inputs).toHaveCount(3);

  await inputs.nth(0).fill('120'); // DAS
  await inputs.nth(1).fill('10'); // ARR
  await inputs.nth(2).fill('20'); // SDF

  await page.reload();
  await expect(inputs.nth(0)).toHaveValue('120');
  await expect(inputs.nth(1)).toHaveValue('10');
  await expect(inputs.nth(2)).toHaveValue('20');

  const stored = await page.evaluate(() => ['das', 'arr', 'sdf'].map((k) => localStorage.getItem(k)));
  expect(stored).toEqual(['120', '10', '20']);
});

test('설정을 바꾼 뒤에도 게임이 정상 시작된다', async ({ page }) => {
  await page.goto('/setting');
  await page.locator('input[type=number]').nth(2).fill('41'); // SDF ∞
  await page.goto('/single');
  await page.getByRole('button', { name: 'Start (R)' }).click();
  await page.keyboard.press('Space');
  await expect(page.locator('#pieces')).toHaveText('1');
});
