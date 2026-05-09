# Tiêu đề (Title)
Ứng dụng DApp Bỏ phiếu kín trên Stellar (voting-contract)

# Mô tả dự án (Description)
Các hệ thống bầu cử truyền thống (cơ chế một-người-một-phiếu) thường không thể hiện được mức độ mong muốn thực sự của người bỏ phiếu đối với một lựa chọn cụ thể. Dự án này được xây dựng nhằm giải quyết hạn chế đó, đặc biệt phù hợp cho môi trường câu lạc bộ trường đại học hoặc tập thể lớp.
Bằng cách triển khai cơ chế Bỏ phiếu kín trên mạng lưới blockchain Stellar (sử dụng ngôn ngữ Rust và Soroban SDK), hợp đồng thông minh này cho phép người bỏ phiếu không chỉ thể hiện họ muốn gì, mà còn mức độ họ khao khát điều đó ra sao. Hệ thống đảm bảo tính minh bạch,  đồng thời ngăn chặn việc một cá nhân hoặc nhóm nhỏ có thể dễ dàng thao túng kết quả.

# Tính năng cốt lõi (Features)
Khởi tạo Bảo mật (Secure Initialization): Hợp đồng gắn quyền quản trị chặt chẽ với một tài khoản Admin (Ban chủ nhiệm) ngay khi triển khai, ngăn chặn mọi hành vi chiếm quyền trái phép.

Cấp phát Tín chỉ (mint_credits): Quản trị viên có thể phân bổ "Điểm tín chỉ bầu cử" (Voting Credits) một cách minh bạch cho các thành viên hợp lệ trong hệ thống.

Chi phí Bỏ phiếu Phi tuyến tính (vote): Áp dụng công thức tính phí toàn phương: Chi phí = (Số phiếu bầu)². Điều này khiến cho việc dồn nhiều phiếu cho một sự kiện trở nên đắt đỏ hơn theo cấp số nhân, đóng vai trò như một cơ chế tự nhiên chống lại sự thao túng (anti-whale mechanism).

Kiểm tra Minh bạch (Transparent Tracking): Các hàm chỉ đọc (get_credits, get_poll_votes) cho phép bất kỳ ai cũng có thể tra cứu số dư tín chỉ của thành viên và cập nhật kết quả bỏ phiếu hiện tại một cách công khai mà không tốn phí mạng lưới (gas fees).

Xác thực Chặt chẽ (Strict Authentication): Kế thừa hàm require_auth() của Soroban để đảm bảo mọi hành động thay đổi dữ liệu đều phải được ký xác nhận bằng mã hóa bởi đúng chủ sở hữu ví.

Liên kết Hợp đồng (Contract)
Liên kết hợp đồng trên Stellar Expert (Mạng Testnet): https://stellar.expert/explorer/testnet/contract/CDOES5GHCFZGMXHIYORMRWOFQIOMNXEK3SUD2BSENNRSCZNSUGFGWKDL

Ảnh chụp màn hình tương tác:
Khởi tạo và Cấp tín chỉ: screenshot_deploy.png
Thực hiện Bỏ phiếu thành công: screenshot_Invoke contract.png

# Hướng phát triển trong tương lai (Future scopes)

Tích hợp Giao diện (Front-end Integration): Xây dựng giao diện Web trực quan bằng React.js và kết nối với ví Stellar Freighter, giúp các thành viên bỏ phiếu dễ dàng mà không cần dùng lệnh Terminal hay nền tảng IDE.

Tích hợp Token thực tế (Real Token Integration): Nâng cấp "Điểm tín chỉ" ảo hiện tại tuân theo chuẩn Soroban Token Interface, cho phép sử dụng các tài sản tiền mã hóa thực (như USDC hoặc XLM) làm chi phí bỏ phiếu, áp dụng cho các mô hình quản trị tổ chức tự trị phi tập trung (DAO) quy mô lớn.

Bỏ phiếu có Thời hạn định kỳ (Time-bound Polls): Thêm logic để hỗ trợ nhiều sự kiện diễn ra cùng lúc với các mốc thời gian bắt đầu và kết thúc (deadline) chặt chẽ, dựa trên dữ liệu khối (ledger sequence) của Stellar.

# Thông tin tác giả (Profile)
Họ và tên: Ngô Phạm Khánh Ngọc
Chức danh: Sinh viên 
Liên hệ/Github: (👉 Điền link Github hoặc Email của bạn vào đây)