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
  public void validateEvents(String events, Promise promise) {
    try {
      String result = validateEventsStr(events);
      promise.resolve(result);
    } catch (Exception e) {
      rejectWithException(promise, "parsing the KEL", e);
    }
  }

  @ReactMethod
  public void getIcp(Promise promise) {
    try {
      String result = getIcpStr();
      promise.resolve(result);
    } catch (Exception e) {
      rejectWithException(promise, "generating the ICP", e);
    }
  }

  @ReactMethod
  public void extractIdFromEvent(String event, Promise promise) {
    try {
      String result = extractIdFromEventStr(event);
      promise.resolve(result);
    } catch (Exception e) {
      rejectWithException(promise, "extracting the ID from key event", e);
    }
  }

  private static native String validateEventsStr(String events);
  private static native String extractIdFromEventStr(String event);
  private static native String getIcpStr();
}
