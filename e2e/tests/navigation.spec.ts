import { test, expect, startGame } from './fixtures';

test('메인 화면에서 싱글·멀티·설정으로 이동한다', async ({ page }) => {
  for (const [name, path] of [
    ['Single Play', '/single'],
    ['Multi Play', '/multi'],
    ['Setting', '/setting'],
  ]) {
    await page.goto('/');
    await page.getByRole('link', { name }).click();
    await expect(page).toHaveURL(new RegExp(`${path}$`));
  }
});

test('게임 화면을 나갔다 다시 들어와도 게임이 동작한다', async ({ page }) => {
  await page.goto('/');
  await page.getByRole('link', { name: 'Single Play' }).click();
  await expect(page.locator('#game-canvas')).toBeVisible();

  await page.goBack();
  await page.getByRole('link', { name: 'Single Play' }).click();
  await expect(page.locator('#game-canvas')).toBeVisible();

  await startGame(page);
  await page.keyboard.press('Space');
  await expect(page.locator('#pieces')).toHaveText('1');
});

test('없는 주소는 메인 화면으로 보낸다', async ({ page }) => {
  await page.goto('/no-such-page');
  await expect(page.getByRole('link', { name: 'Single Play' })).toBeVisible();
});
