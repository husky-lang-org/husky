import wandb
import torch
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader


def train_model(
    model,
    train_dataloader,
    val_dataloader,
    criterion,
    optimizer,
    num_epochs,
    device,
    output_dims,
    log_wandb=True,
    model_name=None,
    padding_value=-1,
    patience=5,
    min_delta=0.001,
):
    ast_dim, symbol_dim, error_dim = output_dims
    model.to(device)

    best_val_loss = float("inf")
    epochs_without_improvement = 0
    best_model_state = None

    for epoch in range(num_epochs):
        # Training phase
        model.train()
        train_loss, train_ast_acc, train_symbol_acc, train_error_acc = run_epoch(
            model,
            train_dataloader,
            criterion,
            optimizer,
            device,
            output_dims,
            padding_value,
            is_training=True,
        )

        # Validation phase
        model.eval()
        val_loss, val_ast_acc, val_symbol_acc, val_error_acc = run_epoch(
            model,
            val_dataloader,
            criterion,
            optimizer,
            device,
            output_dims,
            padding_value,
            is_training=False,
        )

        # Early stopping check
        if val_loss < best_val_loss - min_delta:
            best_val_loss = val_loss
            epochs_without_improvement = 0
            best_model_state = model.state_dict()
        else:
            epochs_without_improvement += 1

        if log_wandb:
            wandb.log(
                {
                    f"{model_name}_train_loss": train_loss,
                    f"{model_name}_train_ast_accuracy": train_ast_acc,
                    f"{model_name}_train_symbol_accuracy": train_symbol_acc,
                    f"{model_name}_train_error_accuracy": train_error_acc,
                    f"{model_name}_val_loss": val_loss,
                    f"{model_name}_val_ast_accuracy": val_ast_acc,
                    f"{model_name}_val_symbol_accuracy": val_symbol_acc,
                    f"{model_name}_val_error_accuracy": val_error_acc,
                },
                step=epoch,
            )

        print(
            f"Epoch [{epoch + 1}/{num_epochs}], "
            f"Train Loss: {train_loss:.4f}, Val Loss: {val_loss:.4f}, "
            f"Train AST Acc: {train_ast_acc:.4f}, Val AST Acc: {val_ast_acc:.4f}, "
            f"Train Symbol Acc: {train_symbol_acc:.4f}, Val Symbol Acc: {val_symbol_acc:.4f}, "
            f"Train Error Acc: {train_error_acc:.4f}, Val Error Acc: {val_error_acc:.4f}"
        )

        if epochs_without_improvement >= patience:
            print(f"Early stopping triggered after {epoch + 1} epochs")
            break

    # Load the best model state
    if best_model_state is not None:
        model.load_state_dict(best_model_state)
        print("Loaded best model state from early stopping")

    return model


def run_epoch(
    model,
    dataloader,
    criterion,
    optimizer,
    device,
    output_dims,
    padding_value,
    is_training,
):
    ast_dim, symbol_dim, error_dim = output_dims
    total_loss = 0.0
    total_ast_acc = 0.0
    total_symbol_acc = 0.0
    total_error_acc = 0.0

    for batch_idx, (inputs, targets) in enumerate(dataloader):
        inputs = inputs.to(device)
        ast_targets, symbol_targets, error_targets = [t.to(device) for t in targets]

        if is_training:
            optimizer.zero_grad()

        with torch.set_grad_enabled(is_training):
            outputs = model(inputs)
            ast_outputs = outputs[:, :, :ast_dim]
            symbol_outputs = outputs[:, :, ast_dim : ast_dim + symbol_dim]
            error_outputs = outputs[
                :, :, ast_dim + symbol_dim : ast_dim + symbol_dim + error_dim
            ]

            ast_outputs = ast_outputs.view(-1, ast_dim)
            symbol_outputs = symbol_outputs.view(-1, symbol_dim)
            error_outputs = error_outputs.view(-1, error_dim)

            ast_targets = ast_targets.view(-1)
            symbol_targets = symbol_targets.view(-1)
            error_targets = error_targets.view(-1)

            ast_mask = ast_targets != padding_value
            symbol_mask = symbol_targets != padding_value
            error_mask = error_targets != padding_value

            ast_loss = criterion(ast_outputs[ast_mask], ast_targets[ast_mask])
            symbol_loss = criterion(
                symbol_outputs[symbol_mask], symbol_targets[symbol_mask]
            )
            error_loss = criterion(error_outputs[error_mask], error_targets[error_mask])

            combined_loss = ast_loss + symbol_loss + error_loss

            if is_training:
                combined_loss.backward()
                optimizer.step()

        total_loss += combined_loss.item()

        ast_acc = (
            (
                (ast_outputs.argmax(dim=1) == ast_targets)
                & (ast_targets != padding_value)
            )
            .float()
            .mean()
        )
        symbol_acc = (
            (
                (symbol_outputs.argmax(dim=1) == symbol_targets)
                & (symbol_targets != padding_value)
            )
            .float()
            .mean()
        )
        error_acc = (
            (
                (error_outputs.argmax(dim=1) == error_targets)
                & (error_targets != padding_value)
            )
            .float()
            .mean()
        )

        total_ast_acc += ast_acc.item()
        total_symbol_acc += symbol_acc.item()
        total_error_acc += error_acc.item()

    avg_loss = total_loss / len(dataloader)
    avg_ast_acc = total_ast_acc / len(dataloader)
    avg_symbol_acc = total_symbol_acc / len(dataloader)
    avg_error_acc = total_error_acc / len(dataloader)

    return avg_loss, avg_ast_acc, avg_symbol_acc, avg_error_acc
