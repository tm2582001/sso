import jwt from "jsonwebtoken";


const verifyJwt = (token)=>{
    const publicKey = process.env.PUBLIC_KEY;
    try{
        const decoded = jwt.verify(token,publicKey);
        return { payload: decoded, expired: null };

    }catch(err){
        console.log(err, "err");
        return { payload: null, expired: err.message };
    }
}

export default verifyJwt;