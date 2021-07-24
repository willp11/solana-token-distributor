const reducer = (state = {wallet: null, walletBtnTxt: "Connect wallet", program_id: "DkhCGxQnQRh7e68ZQjjYABktGi1rG4HEMjdFL45TNQY6"}, action) => {
    switch (action.type) {
        case "startWallet": {
            return {
                ...state,
                wallet: action.wallet,
                walletBtnTxt: "Connected"
            };
        }
        default: {
            return {
                wallet: null, 
                walletBtnTxt: "Connect wallet",
                program_id: "DkhCGxQnQRh7e68ZQjjYABktGi1rG4HEMjdFL45TNQY6"
            }
        }
    }
}

export default reducer;