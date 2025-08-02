const isAuthenticated = (req, res, next) => {
  const redirectURL = `${req.protocol}://${req.headers.host}${req.path}`;
  if (!req.session.user) {
    const loginUrl = req.app.get("loginUrl");
    
    return res.redirect(
      `${loginUrl}?serviceURL=${redirectURL}`
    );
  }
  next();
};

export default isAuthenticated;
