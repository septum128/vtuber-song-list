;<html>

<body>
  <p>{{name}} 様</p>
  <p>
    Vtuber-Song.com にご登録いただき、ありがとうございます。<br>
    下記のリンクからメールアドレスの確認を行ってください。
  </p>
  <p>
    <a href="{{domain}}/api/auth/verify/{{verifyToken}}">
      メールアドレスを確認する
    </a>
  </p>
  <p>今後ともVtuber-Song.comをよろしくお願いいたします。<br>Vtuber-Song.com 運営チーム</p>

  <hr>

  <p>Dear {{name}},</p>
  <p>
    Thank you for registering with Vtuber-Song.com.<br>
    Please verify your email address by clicking the link below.
  </p>
  <p>
    <a href="{{domain}}/api/auth/verify/{{verifyToken}}">
      Verify Your Email Address
    </a>
  </p>
  <p>Best regards,<br>The Vtuber-Song.com Team</p>
</body>

</html>
