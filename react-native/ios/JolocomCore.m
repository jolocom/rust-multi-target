#import <React/RCTBridgeModule.h>

@interface RCT_EXTERN_MODULE (JolocomCore, NSObject)

// explanation about threading here:
// https://stackoverflow.com/a/50775641/3060739
- (dispatch_queue_t)methodQueue {
  return dispatch_get_main_queue();
}

+ (BOOL)requiresMainQueueSetup {
  return YES;
}

// see ./JolocomCore.swift
RCT_EXTERN_METHOD(processEvents
                  : (NSString *)kelString dbPath
                  : (NSString *)dbPath resolve
                  : (RCTPromiseResolveBlock)resolve reject
                  : (RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(resolve_id
                  : (NSString *)id dbPath
                  : (NSString *)dbPath resolve
                  : (RCTPromiseResolveBlock)resolve reject
                  : (RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(getKerl
                  : (NSString *)id dbPath
                  : (NSString *)dbPath resolve
                  : (RCTPromiseResolveBlock)resolve reject
                  : (RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(newWallet
                  : (NSString *)id pass
                  : (NSString *)pass resolve
                  : (RCTPromiseResolveBlock)resolve reject
                  : (RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(keriInceptWallet
                  : (NSString *)ew id
                  : (NSString *)id pass
                  : (NSString *)pass resolve
                  : (RCTPromiseResolveBlock)resolve reject
                  : (RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(keriInceptWalletFromKeys
                  : (NSString *)liveKeys preRotatedKeys
                  : (NSString *)preRotatedKeys pass
                  : (NSString *)pass resolve
                  : (RCTPromiseResolveBlock)resolve reject
                  : (RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(changePass
                  : (NSString *)ew id
                  : (NSString *)id oldPass
                  : (NSString *)oldPass newPass
                  : (NSString *)newPass resolve
                  : (RCTPromiseResolveBlock)resolve reject
                  : (RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(changeId
                  : (NSString *)ew id
                  : (NSString *)id newId
                  : (NSString *)newId pass
                  : (NSString *)pass resolve
                  : (RCTPromiseResolveBlock)resolve reject
                  : (RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(newKey
                  : (NSString *)ew id
                  : (NSString *)id pass
                  : (NSString *)pass type
                  : (NSString *)type controller
                  : (NSString *)controller resolve
                  : (RCTPromiseResolveBlock)resolve reject
                  : (RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(addContent
                  : (NSString *)ew id
                  : (NSString *)id pass
                  : (NSString *)pass content
                  : (NSString *)content resolve
                  : (RCTPromiseResolveBlock)resolve reject
                  : (RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(setKeyController
                  : (NSString *)ew id
                  : (NSString *)id pass
                  : (NSString *)pass keyRef
                  : (NSString *)keyRef controller
                  : (NSString *)controller resolve
                  : (RCTPromiseResolveBlock)resolve reject
                  : (RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(getKey
                  : (NSString *)ew id
                  : (NSString *)id pass
                  : (NSString *)pass keyRef
                  : (NSString *)keyRef resolve
                  : (RCTPromiseResolveBlock)resolve reject
                  : (RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(getKeyByController
                  : (NSString *)ew id
                  : (NSString *)id pass
                  : (NSString *)pass controller
                  : (NSString *)controller resolve
                  : (RCTPromiseResolveBlock)resolve reject
                  : (RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(getKeys
                  : (NSString *)ew id
                  : (NSString *)id pass
                  : (NSString *)pass resolve
                  : (RCTPromiseResolveBlock)resolve reject
                  : (RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(sign
                  : (NSString *)ew id
                  : (NSString *)id pass
                  : (NSString *)pass controller
                  : (NSString *)controller data
                  : (NSString *)data resolve
                  : (RCTPromiseResolveBlock)resolve reject
                  : (RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(verify
                  : (NSString *)key type
                  : (NSString *)type data
                  : (NSString *)data signature
                  : (NSString *)signature resolve
                  : (RCTPromiseResolveBlock)resolve reject
                  : (RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(encrypt
                  : (NSString *)key type
                  : (NSString *)type data
                  : (NSString *)data aad
                  : (NSString *)aad resolve
                  : (RCTPromiseResolveBlock)resolve reject
                  : (RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(decrypt
                  : (NSString *)ew id
                  : (NSString *)id pass
                  : (NSString *)pass controller
                  : (NSString *)controller data
                  : (NSString *)data aad
                  : (NSString *)aad resolve
                  : (RCTPromiseResolveBlock)resolve reject
                  : (RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(getRandom
                  : (NSUInteger *)len resolve
                  : (RCTPromiseResolveBlock)resolve reject
                  : (RCTPromiseRejectBlock)reject)
@end
