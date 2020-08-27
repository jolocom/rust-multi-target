package io.jolocom.jolocomCore;

import com.facebook.react.ReactPackage;
import com.facebook.react.bridge.JavaScriptModule;
import com.facebook.react.bridge.NativeModule;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.uimanager.ViewManager;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;

public class JolocomCorePackage implements ReactPackage {
  @Override
  public List<NativeModule>
  createNativeModules(ReactApplicationContext reactContext) {
    return Arrays.<NativeModule>asList(new JolocomCoreModule(reactContext));
  }

  @Override
  public List<ViewManager>
  createViewManagers(ReactApplicationContext reactContext) {
    return Collections.emptyList();
  }
}
