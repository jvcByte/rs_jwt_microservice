package com.jvcbyte.rsjwt

import android.content.res.Configuration
import android.os.Bundle
import android.webkit.JavascriptInterface
import android.webkit.WebView
import androidx.activity.enableEdgeToEdge
import androidx.core.view.ViewCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat

class MainActivity : TauriActivity() {
  private var webView: WebView? = null
  private var lastInsets: WindowInsetsCompat? = null

  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)

    // Bootstrap the status/nav-bar icon color from the system theme for the
    // brief pre-load frame. Once the WebView reports the real in-app theme
    // (below), that value takes over.
    applyBarAppearance(isNight())

    // The decorView ALWAYS receives the authoritative window insets, even
    // before wry has created the WebView. We capture them here and forward the
    // sizes to the web layer as CSS variables. We return the insets UNMODIFIED
    // (do not consume) so wry's own dispatch is untouched.
    ViewCompat.setOnApplyWindowInsetsListener(window.decorView) { _, insets ->
      lastInsets = insets
      applyInsets()
      insets
    }
  }

  // WryActivity calls this the instant the WebView is attached (wry creates it
  // asynchronously, after onCreate). We keep the reference, wire the theme
  // bridge, paint the load-flash background, and push insets in.
  override fun onWebViewCreate(webView: WebView) {
    this.webView = webView
    // Bridge: the web app reports its resolved theme (the `.dark` class) so the
    // system-bar icons track the IN-APP toggle, not the OS. Registered before
    // wry loads the URL, so it's present in the page's JS context on load.
    webView.addJavascriptInterface(ThemeBridge(), "AndroidBars")
    webView.setBackgroundColor(barStripColor())
    syncWebView()
    ViewCompat.requestApplyInsets(window.decorView)
    // Window insets/theme are pushed at first layout — BEFORE our SPA has
    // loaded, so the first injection lands on about:blank and is lost. A
    // content-only React render does NOT trigger a fresh inset dispatch, so we
    // re-sync on a few delays to catch the app once it has rendered.
    for (delay in longArrayOf(100, 400, 900, 1800, 3000)) {
      webView.postDelayed({ syncWebView() }, delay)
    }
  }

  override fun onResume() {
    super.onResume()
    syncWebView()
  }

  /** Called from JS whenever the in-app theme resolves or changes. Runs on a
   *  binder thread, so hop to the UI thread to touch the window. */
  private inner class ThemeBridge {
    @JavascriptInterface
    fun setDark(dark: Boolean) {
      runOnUiThread { applyBarAppearance(dark) }
    }
  }

  /** Dark status/nav-bar background → light (white) icons; light background →
   *  dark icons. `isAppearanceLight*Bars = true` means "bar bg is light, draw
   *  dark icons", which is what we want when NOT in dark theme. */
  private fun applyBarAppearance(dark: Boolean) {
    val controller = WindowInsetsControllerCompat(window, window.decorView)
    controller.isAppearanceLightStatusBars = !dark
    controller.isAppearanceLightNavigationBars = !dark
  }

  /** Push current insets + (re)install the theme reporter into the WebView. */
  private fun syncWebView() {
    val wv = webView ?: return
    applyInsets()
    wv.evaluateJavascript(THEME_SYNC_JS, null)
  }

  // targetSdk 35+ forces edge-to-edge: the WebView draws behind the status and
  // navigation bars, and Android's WebView never reports those bars through
  // env(safe-area-inset-*). Instead of padding the WebView (which drops the
  // whole app below the bar as a flat strip), we expose the inset sizes as CSS
  // custom properties on :root. The web layout pads its header/nav with them,
  // so those bars' backgrounds extend behind the system bars while only their
  // CONTENTS are inset. Values are converted from physical px to CSS px (dp)
  // and rounded to whole ints (avoids any locale decimal-separator issue).
  private fun applyInsets() {
    val wv = webView ?: return
    val insets = lastInsets ?: return
    val bars = insets.getInsets(
      WindowInsetsCompat.Type.systemBars() or WindowInsetsCompat.Type.displayCutout()
    )
    val d = resources.displayMetrics.density
    val top = Math.round(bars.top / d)
    val right = Math.round(bars.right / d)
    val bottom = Math.round(bars.bottom / d)
    val left = Math.round(bars.left / d)
    val js = """
      (function () {
        var s = document.documentElement.style;
        s.setProperty('--android-inset-top', '${top}px');
        s.setProperty('--android-inset-right', '${right}px');
        s.setProperty('--android-inset-bottom', '${bottom}px');
        s.setProperty('--android-inset-left', '${left}px');
      })();
    """.trimIndent()
    wv.evaluateJavascript(js, null)
  }

  private fun isNight(): Boolean =
    (resources.configuration.uiMode and Configuration.UI_MODE_NIGHT_MASK) ==
      Configuration.UI_MODE_NIGHT_YES

  /** WebView background behind the bars during load, matched to the web app's
   *  `--background` token. Follows the SYSTEM setting (load flash only). */
  private fun barStripColor(): Int =
    if (isNight()) 0xFF0A0A0A.toInt() else 0xFFFFFFFF.toInt()

  companion object {
    // Idempotent: reports the current theme immediately and installs a
    // MutationObserver (once) so a live in-app theme toggle updates the bars.
    private const val THEME_SYNC_JS = """
      (function () {
        function report() {
          try {
            if (window.AndroidBars && window.AndroidBars.setDark) {
              window.AndroidBars.setDark(
                document.documentElement.classList.contains('dark')
              );
            }
          } catch (e) {}
        }
        if (!window.__barThemeSync) {
          window.__barThemeSync = true;
          new MutationObserver(report).observe(document.documentElement, {
            attributes: true,
            attributeFilter: ['class'],
          });
        }
        report();
      })();
    """
  }
}
