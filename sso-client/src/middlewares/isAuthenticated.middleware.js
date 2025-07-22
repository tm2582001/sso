const isAuthenticated = (req, res, next) => {
  const redirectURL = `${req.protocol}://${req.headers.host}${req.path}`;
  if (req.session.user == null) {
    const loginUrl = req.app.get("loginUrl");
    console.log(loginUrl, "here");
    return res.redirect(
      `${loginUrl}?serviceURL=${redirectURL}`
    );
  }
  next();
};

export default isAuthenticated;
