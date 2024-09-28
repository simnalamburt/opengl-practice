ratatui-practice
========
Fix: ([`40d92a2`](https://github.com/simnalamburt/ratatui-practice/commit/40d92a2))

```diff
@@ -28,7 +28,7 @@ impl State {
     }

     fn cursor(&self) -> u16 {
-        self.idx_chars as u16
+        self.input[..self.idx_bytes()].width_cjk() as u16
     }
 }
```

Before:

![](./before.avif)

After:

![](./after.avif)
