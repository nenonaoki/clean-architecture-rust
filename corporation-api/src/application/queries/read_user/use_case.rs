use super::dto::{Error, ErrorStatus, Input, Output};

pub fn interactor(input: Input) -> Result<Output, Error> {
    // TODO: 実際のリポジトリからユーザーを取得するロジックを実装
    // 現在は入力データをそのまま返すダミー実装

    // エラーケースのテスト（実際の実装では適切な条件でエラーを返す）
    if input.user_name.is_empty() {
        return Err(Error {
            status: ErrorStatus::Unknown,
        });
    }

    Ok(Output {
        user_name: input.user_name,
    })
}
