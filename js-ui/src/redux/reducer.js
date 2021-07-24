const reducer = (state = {wallet: null, walletBtnTxt: "Connect wallet"}, action) => {
    switch (action.type) {
        case "startWallet": {
            return {
                wallet: action.wallet,
                walletBtnTxt: "Connected"
            };
        }
        default: {
            return {
                wallet: null, 
                walletBtnTxt: "Connect wallet"
            }
        }
    }
}

export default reducer;