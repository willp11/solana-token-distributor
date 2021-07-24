import './createLockupSchedule.css';
import { useSelector } from "react-redux";
import { useState } from 'react';
import { createLockupSchedule } from '../../util/createLockupSchedule';

const CreateLockupSchedule = () => {

    const reduxState = useSelector(state => state);

    const [formData, setFormData] = useState({
        startTimestamp: null,
        totalUnlockPeriods: null,
        periodDuration: null,
        totalLockupQuantity: null,
        tokenMintPubkey: null
    })

    const inputHandler = (e) => {
        let newValue = e.target.value;
        // if date, convert to seconds
        if (e.target.name == "startTimestamp") {
            let timestamp = new Date(newValue).getTime();
            newValue = timestamp / 1000;
        }
        let state = {...formData};
        state[e.target.name] = newValue;
        setFormData(state);
    }

    const submitHandler = async () => {
        if (reduxState.wallet == null) {
            alert("Connect to wallet");
        } else {
            const lockupSchedule = await createLockupSchedule(
                reduxState.program_id,
                reduxState.wallet,
                formData.startTimestamp,
                formData.totalUnlockPeriods,
                formData.periodDuration,
                formData.totalLockupQuantity,
                formData.tokenMintPubkey
            );
            console.log(lockupSchedule);
        }
    }
    
    return (
        <div className="CreateLockupSchedule">
            <h2>Create Lockup Schedule</h2>
            <h3>Start Date and Time (your local time)</h3>
            <input type="datetime-local" name="startTimestamp" onChange={e=>inputHandler(e)}/> <br/>
            <input type="number" name="totalUnlockPeriods" placeholder="total unlock periods" onChange={e=>inputHandler(e)}/> <br/>
            <input type="number" name="periodDuration" placeholder="period duration (seconds)" onChange={e=>inputHandler(e)}/> <br/>
            <input type="number" name="totalLockupQuantity" placeholder="total lockup quantity" onChange={e=>inputHandler(e)}/> <br/>
            <input type="text" name="tokenMintPubkey" placeholder="token mint public key (base58)" onChange={e=>inputHandler(e)}/> <br/>
            <button onClick={submitHandler}>Send tx</button>
        </div>
    );
}

export default CreateLockupSchedule;