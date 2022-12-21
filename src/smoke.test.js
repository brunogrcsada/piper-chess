// Test that Jest is working

describe("Smoke Test", () => {
  it("should confirm that two strings are equal", () => {
    expect("hello world").toBe("hello world");
    expect("think different").toBe("think different");
  });
});
