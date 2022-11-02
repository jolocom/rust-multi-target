package io.jolocom.jolocomCore;

import com.facebook.react.bridge.Promise;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactContextBaseJavaModule;
import com.facebook.react.bridge.ReactMethod;

public class JolocomCoreModule extends ReactContextBaseJavaModule {

  private final ReactApplicationContext reactContext;

  static { System.loadLibrary("keriox_wrapper"); }

  public JolocomCoreModule(ReactApplicationContext reactContext) {
    super(reactContext);
    this.reactContext = reactContext;
  }

  private void rejectWithException(Promise promise, String code, Exception e) {
    String[] sp = e.getMessage().split(": ");
    String s = sp[sp.length - 1].trim().replace("\"", "");
    promise.reject(code, s);
  }

  @Override
  public String getName() {
    return "JolocomCore";
  }

  @ReactMethod
  public void createIdentityWithConfig(String config, Promise promise) {
    try {
      String result = createIdentityWithConfig(config);
      promise.resolve(result);
    } catch (Exception e) {
      rejectWithException(promise, "Could not create identity", e);
    }
  }

  @ReactMethod
  public void createIdentity(String path, Promise promise) {
    try {
      String result = createIdentity(path);
      promise.resolve(result);
    } catch (Exception e) {
      rejectWithException(promise, "Could not create identity", e);
    }
  }

  private static native String createIdentity(String path);
  private static native String createIdentityWithConfig(String config);
}
