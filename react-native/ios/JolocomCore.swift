import Foundation

func handle_error<T, U>(
  resolve: RCTPromiseResolveBlock,
  reject: RCTPromiseRejectBlock,
  get_result: (UnsafeMutablePointer<ExternError>) -> T,
  success: (T) -> U
) -> Void {
  var err = ExternError()
  let err_ptr: UnsafeMutablePointer<ExternError> = UnsafeMutablePointer(&err)
  let res = get_result(err_ptr)
  if err_ptr.pointee.code == 0 {
    resolve(success(res))
  } else {
    let val = String(cString: err_ptr.pointee.message)
    jolo_destroy_string(err_ptr.pointee.message)
    reject(String(describing: err_ptr.pointee.code), val, nil)
  }
}

@objc(JolocomCore)
class JolocomCore: NSObject {

  public static func requiresMainQueueSetup() -> Bool {
    return true;
  }

// validates a key event log,
// see ./core.h:34
  @objc func validateEvents(_ kelString: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> String {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { validate_events($0, kelString) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        jolo_destroy_string(res!)
        return val
    })
  }

  @objc func keriInceptWallet(_ ew: String, id: String, pass: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> String {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { keri_incept_wallet($0, ew, id, pass) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        jolo_destroy_string(res!)
        return val
    })

  @objc func getIdFromEvent(_ event: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> String {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { get_id_from_event($0, event) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        jolo_destroy_string(res!)
        return val
    })
  }

  @objc func newWallet(_ id: String, pass: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> String {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { new_wallet($0, id, pass) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        jolo_destroy_string(res!)
        return val
    })
  }

  @objc func changePass(_ ew: String, id: String, oldPass: String, newPass: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> String {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { change_pass($0, ew, id, oldPass, newPass) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        jolo_destroy_string(res!)
        return val
    })
  }


  @objc func changeId(_ ew: String, id: String, newId: String, pass: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> String {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { change_id($0, ew, id, new_id, pass) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        jolo_destroy_string(res!)
        return val
    })
  }

  @objc func newKey(_ ew: String, id: String, pass: String, keyType: String, controller: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> String {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { new_key($0, ew, id, pass, keyType, controller) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        jolo_destroy_string(res!)
        return val
    })
  }

  @objc func addContent(_ ew: String, id: String, pass: String, content: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> String {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { add_content($0, ew, id, pass, content) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        jolo_destroy_string(res!)
        return val
    })
  }

  @objc func getKey(_ ew: String, id: String, pass: String, keyRef: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> String {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { get_key($0, ew, id, pass, keyRef) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        jolo_destroy_string(res!)
        return val
    })
  }

  @objc func getKeyByController(_ ew: String, id: String, pass: String, controller: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> String {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { get_key_by_controller($0, ew, id, pass, controller) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        jolo_destroy_string(res!)
        return val
    })
  }

  @objc func setKeyByController(_ ew: String, id: String, pass: String, keyRef: String, controller: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> String {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { set_key_by_controller($0, ew, id, pass, keyRef, controller) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        jolo_destroy_string(res!)
        return val
    })
  }

  @objc func getKeys(_ ew: String, id: String, pass: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> String {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { get_keys($0, ew, id, pass) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        jolo_destroy_string(res!)
        return val
    })
  }

  @objc func sign(_ ew: String, id: String, pass: String, controller: string, data: string, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> String {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { sign($0, ew, id, pass, controller, data) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        jolo_destroy_string(res!)
        return val
    })
  }

  @objc func decrypt(_ ew: String, id: String, pass: String, controller: string, data: string, aad: String, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> String {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { decrypt($0, ew, id, pass, controller, data, aad) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        jolo_destroy_string(res!)
        return val
    })
  }

  @objc func verify(_ key: String, keyType: String, data: String, signature: string, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> String {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { verify($0, key, keyType, data, signature) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        jolo_destroy_string(res!)
        return val
    })
  }

  @objc func encrypt(_ key: String, keyType: String, data: String, aad: string, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> String {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { encrypt($0, key, keyType, data, aad) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        jolo_destroy_string(res!)
        return val
    })
  }

  @objc func getRandom(_ len: NSUInteger, resolve: RCTPromiseResolveBlock, reject: RCTPromiseRejectBlock) -> String {
    handle_error(
      resolve: resolve,
      reject: reject,
      get_result: { get_random($0, len) },
      success: { (res: Optional<UnsafePointer<CChar>>) -> String in
        let val = String(cString: res!)
        jolo_destroy_string(res!)
        return val
    })
  }
}
