import {URL} from "node:url";
import verifyJwt from "../utils/verifyJwt.util.js";


async function ssoRedirect(req, res, next){
    const {ssoToken} = req.query;

    console.log(ssoToken);

    if(!ssoToken){
        return next();
    }
    
    const redirectUrl = new URL(`${req.protocol}://${req.get('host')}${req.originalUrl}`).pathname;

    try{
        const jwtUrl = req.app.get("jwtUrl");
        console.log(`${jwtUrl}?ssoToken=${ssoToken}`);

        const response = await fetch(`${jwtUrl}?ssoToken=${ssoToken}`, {
            headers: {
                Authorization: "Bearer 1234"
            }
        });

        if(!response.ok){
            return next();
        }

        const responseJson = await response.json();

        const {token} = responseJson;

        const {payload, error} = verifyJwt(token);
        if(error) {
            return next(error);
        }

        req.session.user = payload;

        return res.redirect(redirectUrl);
    }catch(err){
        return next(err);
    }
}

export default ssoRedirect;