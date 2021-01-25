# #!/usr/bin/python3

from libs.libkeriox_wrapper import Wallet, Error as WalletError 

wallet = Wallet.new_wallet("did", "pass")
error_ocuured = False
try :
    Wallet.keri_incept_wallet(wallet, "did", "pass")
except WalletError:
    error_ocuured = True

assert not error_ocuured

wallet = Wallet.change_pass(wallet, "did", "pass", "new_pass")

try :
    Wallet.add_content(wallet, "did", "pass", "content")
except WalletError:
    error_ocuured = True

assert error_ocuured