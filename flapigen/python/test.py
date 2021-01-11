# #!/usr/bin/python3

from libs.libkeriox_wrapper import Wallet, Error as WalletError 

event = '{"v":"KERI10JSON000144_","i":"EIP6JVp53VZau8wPba8mMsc8wRH1eySlFvCOuMmxm_-w","s":"0","t":"icp","kt":"2","k":["DSuhyBcPZEZLK-fcw5tzHn2N46wRCG_ZOoeKtWTOunRA","DVcuJOOJF1IE8svqEtrSuyQjGTd2HhfAkt9y2QkUtFJI","DT1iAhBWCkvChxNWsby2J0pJyxBIxbAtbLA0Ljx-Grh8"],"n":"E9izzBkXX76sqt0N-tfLzJeRqj0W56p4pDQ_ZqNCDpyw","wt":"0","w":[],"c":[]}-AADAAPGMuShtKCzc_oXMViVhVPMkAfmSyeRtQ2EHtvgRy2NrcXbx-o-vAOwMVv2gOit2JetBrIpJ9Vrk7AIqlUUfCAQABq8xN8U7XetGS5ayeX7dd9jZ4dTdNSBcF4Ov57k0TzdBh5ukwi3ocpY77qZ4vr0nlK83iIbLBu999UZ7XoljBBgACKJuVmv5usbWsDOjq_I8028pXe0Nib2YlKgKKtx1AblOCtWBU2zD_qgXUs58ACmOcFIdTfkxhR3u_jCN8XfHSAw'
assert Wallet.get_id_from_event(event) == 'EIP6JVp53VZau8wPba8mMsc8wRH1eySlFvCOuMmxm_-w'

wallet = Wallet.new_wallet("aaa", "bbb")
error_ocuured = False
try :
    Wallet.keri_incept_wallet(wallet, "aaa", "bbb")
except WalletError:
    error_ocuured = True
    print("Error in `keri_incept_wallet`")

assert not error_ocuured

wallet = Wallet.change_pass(wallet, "aaa", "bbb", "ccc")

try :
    Wallet.add_content(wallet, "aaa", "bbb", "content")
except WalletError:
    error_ocuured = True
    print("Wrong password in `add_content`")

assert error_ocuured