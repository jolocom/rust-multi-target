package io.jolocom.nativeUtils;

import com.facebook.react.bridge.Promise;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactContextBaseJavaModule;
import com.facebook.react.bridge.ReactMethod;

public class NativeUtilsModule extends ReactContextBaseJavaModule {

  private final ReactApplicationContext reactContext;

  static { System.loadLibrary("keriox_wrapper"); }

  public NativeUtilsModule(ReactApplicationContext reactContext) {
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
    return "NativeUtils";
  }

  @ReactMethod
  public void keriValidateEvents(String events, Promise promise) {
    try {
      String result = validateEvents(events);
      promise.resolve(result);
    } catch (Exception e) {
      rejectWithException(promise, "parsing the KEL", e);
    }
  }

  @ReactMethod
  public void keriGetIcp(String sk, String pk, Promise promise) {
    try {
      String result = getIcp(sk, pk);
      promise.resolve(result);
    } catch (Exception e) {
      rejectWithException(promise, "generating the ICP", e);
    }
  }

  private static native String validateEvents(String events);
  private static native String getIcp(String sk, String pk);
}
