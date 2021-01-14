# #!/usr/bin/python3

from libs.libkeriox_wrapper import Wallet, Error as WalletError 

wallet = Wallet.new_wallet("aaa", "bbb")
error_ocuured = False
try :
    Wallet.keri_incept_wallet(wallet, "aaa", "bbb")
except WalletError:
    error_ocuured = True

assert not error_ocuured

wallet = Wallet.change_pass(wallet, "aaa", "bbb", "ccc")

try :
    Wallet.add_content(wallet, "aaa", "bbb", "content")
except WalletError:
    error_ocuured = True

assert error_ocuured