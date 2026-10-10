import { test, expect, openSingle, startGame } from './fixtures';

test.beforeEach(async ({ page }) => {
  await openSingle(page);
});

test('시작하면 시간이 흐른다', async ({ page }) => {
  await startGame(page);
  await expect.poll(() => page.locator('#time').textContent()).not.toBe('0.00');
});

test('하드 드롭할 때마다 조각 수가 하나씩 오른다', async ({ page }) => {
  await startGame(page);
  for (const expected of ['1', '2', '3']) {
    await page.keyboard.press('Space');
    await expect(page.locator('#pieces')).toHaveText(expected);
  }
});

test('Space가 포커스된 시작 버튼을 다시 누르지 않는다', async ({ page }) => {
  await startGame(page);
  await expect(page.getByRole('button', { name: 'Start (R)' })).toBeFocused();
  await page.keyboard.press('Space');
  await page.keyboard.press('Space');
  // 버튼이 눌려 새 판이 시작됐다면 0으로 돌아간다.
  await expect(page.locator('#pieces')).toHaveText('2');
});

test('R 키로 새 판을 시작한다', async ({ page }) => {
  await startGame(page);
  await page.keyboard.press('Space');
  await page.keyboard.press('Space');
  await expect(page.locator('#pieces')).toHaveText('2');

  await page.keyboard.press('KeyR');
  await expect(page.locator('#pieces')).toHaveText('0');
  await expect(page.locator('#lines')).toHaveText('0');
});

test('제자리 하드 드롭만 반복하면 게임 오버가 된다', async ({ page }) => {
  await startGame(page);
  await expect(async () => {
    await page.keyboard.press('Space');
    await expect(page.locator('#message')).toHaveText('Game Over', { timeout: 50 });
  }).toPass({ timeout: 30_000 });

  // 끝난 판은 입력을 받지 않는다.
  const pieces = await page.locator('#pieces').textContent();
  await page.keyboard.press('Space');
  await expect(page.locator('#pieces')).toHaveText(pieces!);
});

test('방향키가 페이지를 스크롤하지 않는다', async ({ page }) => {
  await page.setViewportSize({ width: 1280, height: 400 });
  await startGame(page);
  // 버튼을 누르려고 Playwright가 스크롤했을 수 있으니 그 위치를 기준으로 본다.
  const before = await page.evaluate(() => window.scrollY);
  await page.keyboard.press('ArrowDown');
  await page.keyboard.press('Space');
  await page.keyboard.press('ArrowUp');
  expect(await page.evaluate(() => window.scrollY)).toBe(before);
});
